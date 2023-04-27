use std::{collections::HashMap, future::Future, error::Error};
use futures::StreamExt;
use ndarray::{Array, Array1};
use futures::FutureExt;
use crate::model;

type InferenceKey = String;
type InferResponse = Result<(InferenceKey, triton_client::inference::ModelInferResponse), triton_client::client::Error>;

enum Embeddable {
    SearchResult(model::fatcat::SearchResult),
    Paper(model::Paper),
}

pub fn extract_query_string_from_search_result(paper: model::fatcat::SearchResult) -> String {
    let extra_info = paper
        .abstracts
        .first()
        .map(|a| a.body.clone())
        .unwrap_or(paper
            ._highlights
            .first()
            .unwrap_or(&"".to_string())
            .clone());
    // python reference:
    // text batch = [d['title'] + tokenizer.sep_token + (d.get('abstract') or '') for d in papers]
    let string = paper.biblio.title.clone() + "[SEP]" + extra_info.as_str();
    string
}

pub fn extract_query_string_from_paper(paper: model::Paper) -> String {
    // python reference:
    // text batch = [d['title'] + tokenizer.sep_token + (d.get('abstract') or '') for d in papers]
    let string = paper.title.clone() + "[SEP]" + &paper.r#abstract;
    string
}

pub fn extract_query_string(from: Embeddable) -> String {
    match from {
        SearchResult(res) => extract_query_string_from_search_result(res),
        Paper(paper) => extract_query_string_from_paper(paper) 
    }
}


pub async fn single_request(client: &triton_client::Client, tokenizer: &tokenizers::Tokenizer, string: String, key: InferenceKey) -> InferResponse {
    type TokenInT = i64;

    let encoding = tokenizer.encode(string, false);
    let tokens = match encoding {
        Ok(encoding) => encoding.get_ids()[..std::cmp::min(encoding.len(), 512)].iter().map(|i| *i as TokenInT).collect::<Vec<TokenInT>>(),
        Err(e) => {
            println!("Error encoding: {:?}", e);
            vec![]
            },
    };

    // need to pad to 512
    // save length first for attention mask
    let unmasked = tokens.len();

    // generate attention mask
    let attention_mask_content = (0..unmasked).map(|_| 1).collect::<Vec<TokenInT>>();

    let input_ids = triton_client::inference::model_infer_request::InferInputTensor {
        name: "input_ids".into(),
        shape: vec![1, unmasked as i64],
        parameters: HashMap::new(),
        datatype: "INT64".into(),
        contents: None, //Some(triton_client::inference::InferTensorContents{uint_contents: tokens, .. Default::default()}),
    };

    let attention_mask = triton_client::inference::model_infer_request::InferInputTensor {
        name: "attention_mask".into(),
        shape: vec![1, unmasked as i64],
        parameters: HashMap::new(),
        datatype: "INT64".into(),
        contents: None, //Some(triton_client::inference::InferTensorContents{uint_contents: attention_mask, .. Default::default()}),
    };

    let inputs: Vec<triton_client::inference::model_infer_request::InferInputTensor> = vec![input_ids, attention_mask];
    // little endian bytes as per triton documentation
    let raw_input_id_contents = tokens.iter().map(|t| t.to_le_bytes().to_vec()).flatten().collect::<Vec<u8>>();
    let raw_attention_mask_contents = attention_mask_content.iter().map(|t| t.to_le_bytes().to_vec()).flatten().collect::<Vec<u8>>();
    let raw_input_contents = vec![raw_input_id_contents, raw_attention_mask_contents];

    let req = triton_client::inference::ModelInferRequest {
        id: "".into(),
        model_name: "specter_proximity".into(),
        model_version: 1.to_string(),
        parameters: HashMap::new(),
        inputs,
        outputs: vec![triton_client::inference::model_infer_request::InferRequestedOutputTensor {
            name: "embedding".into(),
            parameters: HashMap::new(),
        }],
        raw_input_contents,
    };
    
    match client.model_infer(req).await {
        Ok(response) => {
            Ok((key, response))
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn parse_inference_response(response: InferResponse) -> Result<(InferenceKey, Vec<Array1<f32>>), Box<dyn Error + Send + Sync>> {
    match response {
        Ok((paper_key, response)) => {
            let vectors = response.raw_output_contents.iter().map(|r| {
                let e = bytemuck::cast_slice::<u8, f32>(r.as_slice());
                let out: Array1<f32> = Array::from_vec(e.to_vec());
                out
            }).collect::<Vec<Array1<f32>>>();
            Ok((paper_key, vectors))
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(Box::new(e))
        }
    }
}


pub async fn get_inference_stream<'a>(client: &'a triton_client::Client, tokenizer: &'a tokenizers::Tokenizer, search_response: model::fatcat::SearchResponse) -> impl futures::stream::Stream<Item = impl Future<Output = InferResponse> + 'a> {
    futures::stream::iter(search_response.results)
    .map(move |r| {
        let key = r.collapse_key.clone();
        single_request(client, tokenizer, extract_query_string_from_search_result(r), key)
    })
}

pub async fn get_embeddings_buffered<'a>(client: &'a triton_client::Client, tokenizer: &'a tokenizers::Tokenizer, search_response: model::fatcat::SearchResponse) -> impl futures::stream::Stream<Item = (String, Array1<f32>)> + 'a {
    get_inference_stream(client, tokenizer, search_response).flatten_stream().buffered(32).map(|r| {
        parse_inference_response(r)
    }).buffered(100).filter_map(|r| async {
        match r {
            Ok((paper_key, vectors)) => {
                match vectors.first() {
                    Some(v) => Some((paper_key, v.clone())),
                    None => None,
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    })
}