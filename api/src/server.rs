use std::collections::HashMap;

use futures::StreamExt;
use qdrant_client::prelude::Payload;
use qdrant_client::qdrant::PointId;
use qdrant_client::qdrant::PointStruct;
use qdrant_client::qdrant::SearchPoints;
use qdrant_client::qdrant::Value;
use qdrant_client::qdrant::WithPayloadSelector;
use qdrant_client::qdrant::WithVectorsSelector;
use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::read_consistency;
use qdrant_client::qdrant::with_payload_selector;
use qdrant_client::qdrant::with_vectors_selector;
use tonic::{transport::Server, Request, Response, Status};
use crate::handlers::embedding;
use crate::handlers::extract_query_string_from_paper;
use crate::handlers::extract_query_string_from_search_result;
use crate::handlers::single_request;
use crate::model;
use crate::model::Paper;
use crate::model::SearchResult;
use crate::rank;
use surface::paper_search_server::{PaperSearch, PaperSearchServer};
use surface::{GetPaperRequest, GetPaperResponse, InsertPaperRequest, InsertPaperResponse, SearchPaperRequest, SearchPaperResponse, PaperVec};

use self::surface::Externalids;

pub mod surface {
    tonic::include_proto!("surface"); // The string specified here must match the proto package name
}


//#[derive(Debug)]
pub struct PaperSearchEngine {
    pub inference_client: triton_client::Client,
    pub tokenizer: tokenizers::Tokenizer,
    pub fatcat_client: reqwest::Client,
    pub qdrant_client: qdrant_client::client::QdrantClient,
}

#[tonic::async_trait]
impl PaperSearch for PaperSearchEngine {


    async fn search_paper(&self, request: Request<SearchPaperRequest>) -> Result<Response<SearchPaperResponse>, Status> {

        let r = request.into_inner();
        let query = r.query;
        let limit = r.limit;

        let embedding_request = single_request(&self.inference_client, &self.tokenizer, query, "search-query".to_string());

        let selector_options = Some(with_vectors_selector::SelectorOptions::Enable(true));
        let payload_selector = Some(with_payload_selector::SelectorOptions::Enable(true));

        let res = match embedding_request.await {
            Ok(res) => res,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let res = crate::embedding::parse_inference_response(Ok(res)).await.unwrap();
        let vector = res.1.first().unwrap();

        let search_query = self.qdrant_client.search_points(&SearchPoints {
            collection_name: "papers".to_string(),
            vector: vector.to_vec(),
            vector_name: None,
            limit: limit,
            score_threshold: Some(0.2f32),
            params: None,
            offset: None,
            filter: None,
            with_payload: Some(WithPayloadSelector{selector_options: payload_selector}),
            with_vectors: None,
            ..Default::default()
        }).await.unwrap();

        let payload: Vec<HashMap<String, Value>> = search_query.result.iter().map(|r| r.payload.clone()).collect();
 
        let papers: Vec<surface::Paper> = payload.iter().map(|p|{
            let paper: Paper = p.clone().into();
            surface::Paper {
                corpusid: paper.corpusid.to_string(),
                openaccessinfo: Some(surface::OpenAccessInfo {
                    externalids: Some(surface::Externalids {
                        mag: paper.openaccessinfo.externalids.mag.unwrap_or_default(),
                        acl: paper.openaccessinfo.externalids.acl.unwrap_or_default(),
                        arxiv: paper.openaccessinfo.externalids.arxiv.unwrap_or_default(),
                        doi: paper.openaccessinfo.externalids.doi.unwrap_or_default(),
                        pmid: paper.openaccessinfo.externalids.pubmedcentral.unwrap_or_default(),
                    }),
                    license: paper.openaccessinfo.license.unwrap_or_default(),
                    url: paper.openaccessinfo.url.unwrap_or_default(),
                    status: paper.openaccessinfo.status.unwrap_or_default(),
                }),
                title: paper.title,
                r#abstract: paper.r#abstract,
                updated: paper.updated.to_rfc3339(),
                fulltext: "".into(),
            }
        }).collect();

        Ok(Response::new(SearchPaperResponse {
            papers: papers
        }))
        
    }

    // Old implementation (reranking of archive scholar)

    // async fn search_paper(&self, request: Request<SearchPaperRequest>) -> Result<Response<SearchPaperResponse>, Status> {
    //     println!("Handling Request: {:?}", request);
    //     let r = request.into_inner();
    //     //println!("request: {:?}", r);
    //     let query = r.query;
    //     let limit = r.limit;

    //     let res = self.fatcat_client.get("https://scholar.archive.org/search").query(&[("q", &query), ("limit", &limit.to_string())]).send().await;
    //     let res = match res {
    //         Ok(res) => res,
    //         Err(e) => {
    //             println!("fatcat error: {:?}", e);
    //             return Err(Status::internal(e.to_string()))
    //         },
    //     };

    //     //println!("res: {:?}", res);
    //     let search_response = res.json::<model::fatcat::SearchResponse>().await;
    //     let response_objects = match search_response {
    //         Ok(res) => res,
    //         Err(e) => return Err(Status::internal(e.to_string())),
    //     };

    //     let query_embed = crate::embedding::single_request(&self.inference_client, &self.tokenizer, query, "query".into());
    //     let embeds = crate::embedding::get_embeddings_buffered(&self.inference_client, &self.tokenizer, response_objects.clone()).await;
        
    //     let mut vec = embeds.collect::<Vec<(String, ndarray::Array1<f32>)>>().await;

    //     let query_embed = match query_embed.await {
    //         Ok(q) => crate::embedding::parse_inference_response(Ok(q)).await,
    //         Err(e) => return Err(Status::internal(e.to_string())),
    //     };
        
    //     rank::rank_by(query_embed.unwrap().1.first().unwrap(), &mut vec, rank::cosine_similarity);

    //     let res = vec[0..std::cmp::min(r.top_k as usize, vec.len())].to_vec();

    //     let papers: Vec<surface::Paper> = res.iter().map(|(collapse_key, _)| {
    //         let cor = response_objects.results.iter().find(|r| r.collapse_key == *collapse_key).unwrap();
    //         cor.into()
    //     }).collect();
    //     Ok(Response::new(SearchPaperResponse { papers }))
    // }

    async fn get_paper(&self, request: Request<GetPaperRequest>) -> Result<Response<GetPaperResponse>, Status> {
        let r = request.into_inner();
        let corpusid = r.corpusid;
        let paperid = corpusid.parse::<u64>().unwrap();
        let selector_options = Some(with_vectors_selector::SelectorOptions::Enable(true));
        let payload_selector = Some(with_payload_selector::SelectorOptions::Enable(true));
        let point_id = PointId {
            point_id_options: Some(PointIdOptions::Num(paperid)),
        };

        let res = self.qdrant_client.get_points("papers", &vec![point_id], Some(WithVectorsSelector{selector_options}), Some(WithPayloadSelector{selector_options: payload_selector}), None).await;
        let res = match res {
            Ok(res) => res,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let payload: Vec<HashMap<String, Value>> = res.result.iter().map(|r| r.payload.clone()).collect();

        match payload.len() {
            0 => return Err(Status::not_found("paper not found")),
            _ => {
                let model_paper: Paper = payload[0].clone().into();
                let get_paper_response = GetPaperResponse {
                    paper: Some(surface::Paper {
                        corpusid: model_paper.corpusid.to_string(),
                        openaccessinfo: Some(surface::OpenAccessInfo {
                            externalids: Some(surface::Externalids {
                                mag: model_paper.openaccessinfo.externalids.mag.unwrap_or_default(),
                                acl: model_paper.openaccessinfo.externalids.acl.unwrap_or_default(),
                                arxiv: model_paper.openaccessinfo.externalids.arxiv.unwrap_or_default(),
                                doi: model_paper.openaccessinfo.externalids.doi.unwrap_or_default(),
                                pmid: model_paper.openaccessinfo.externalids.pubmedcentral.unwrap_or_default(),
                            }),
                            license: model_paper.openaccessinfo.license.unwrap_or_default(),
                            url: model_paper.openaccessinfo.url.unwrap_or_default(),
                            status: model_paper.openaccessinfo.status.unwrap_or_default(),
                        }),
                        title: model_paper.title,
                        r#abstract: model_paper.r#abstract,
                        updated: model_paper.updated.to_rfc3339(),
                        fulltext: "".into(),
                    })
                };

                Ok(Response::new(get_paper_response))
            },
        }
        
    }

    async fn insert_paper(&self, request: Request<InsertPaperRequest>) -> Result<Response<InsertPaperResponse>, Status> {

        let r = request.into_inner();
        let paper = r.paper;
        let paper = paper.unwrap();

        // pub struct Paper {
        //     pub corpusid: u64,
        //     pub openaccessinfo: OpenAccessInfo,
        //     pub title: String,
        //     pub r#abstract: String,
        //     pub updated: chrono::DateTime<Utc>,
        // }
        
        let paperid = paper.corpusid.parse::<u64>().unwrap();

        let internal_paper_representation = model::Paper {
            corpusid: paperid,
            openaccessinfo: paper.openaccessinfo.unwrap().into(),
            title: paper.title,
            r#abstract: paper.r#abstract,
            updated: chrono::DateTime::parse_from_rfc3339(&paper.updated).unwrap_or_default().into(), // .parse::<chrono::DateTime<chrono::Utc>>().unwrap(),
        };

        let payload: Payload = internal_paper_representation.clone().into();

        let query_string = extract_query_string_from_paper(internal_paper_representation);
        let res = single_request(&self.inference_client, &self.tokenizer, query_string, paperid.to_string()).await;

        let res = match res {
            Ok(res) => res,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let res = crate::embedding::parse_inference_response(Ok(res)).await.unwrap();
        let vector = res.1.first().unwrap();

        let points = vec![PointStruct::new(paperid, vector.to_vec(), payload)];

        let insert_result = self.qdrant_client
        .upsert_points_blocking("papers", points, None)
        .await;
        
        match insert_result {
            Ok(res) => res,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let insert_paper_response = InsertPaperResponse {
            corpusid: paper.corpusid,
        };
        
        Ok(Response::new(insert_paper_response))
    }
}