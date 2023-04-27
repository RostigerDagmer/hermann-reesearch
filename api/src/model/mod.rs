//pub mod paper;
//pub use paper::*;

pub mod fatcat;
use std::collections::HashMap;

pub use fatcat::*;

use chrono::Utc;
use qdrant_client::{prelude::Payload, qdrant::Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Externalids {
    pub mag: Option<String>,
    pub acl: Option<String>,
    pub doi: Option<String>,
    pub pubmedcentral: Option<String>,
    pub arxiv: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OpenAccessInfo {
    pub externalids: Externalids,
    pub license: Option<String>,
    pub url: Option<String>,
    pub status: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Paper {
    pub corpusid: u64,
    pub openaccessinfo: OpenAccessInfo,
    pub title: String,
    pub r#abstract: String,
    pub updated: chrono::DateTime<Utc>,
}

// fn extract_integer_value(payload: &Payload, key: &str) -> i64 {
//     let value = <Payload as Into<HashMap<String, Value>>>::into(*payload).get(key).unwrap().kind.unwrap();
//     let value = match value {
//             qdrant_client::qdrant::value::Kind::IntegerValue(value) => value,
//             _ => panic!("value {:?} was not an integer", key),
//     };
//     value
// }
// fn extract_string_value(payload: &Payload, key: &str) -> String {
//     let value = <Payload as Into<HashMap<String, Value>>>::into(*payload).get(key).unwrap().kind.unwrap();
//     let value = match value {
//             qdrant_client::qdrant::value::Kind::StringValue(value) => value,
//             _ => panic!("value {:?} was not an integer", key),
//     };
//     value
// }

// fn extract_struct_value(payload: &Payload, key: &str) -> HashMap<String, Value> {
//     let value = <Payload as Into<HashMap<String, Value>>>::into(*payload).get(key).unwrap().kind.unwrap();
//     let value = match value {
//             qdrant_client::qdrant::value::Kind::StructValue(value) => value,
//             _ => panic!("value {:?} was not an integer", key),
//     };
//     value.fields
// }

impl From<HashMap<String, Value>> for Paper {
    fn from(map: HashMap<String, Value>) -> Self {
        let mut payload = map;
        let corpusid = payload.get("corpusid").cloned().unwrap().kind.unwrap();
        let corpusid = match corpusid {
            qdrant_client::qdrant::value::Kind::IntegerValue(value) => value,
            _ => panic!("value {:?} was not an integer", "corpusid"),
        };

        let title = payload.get("title").cloned().unwrap().kind.unwrap();
        let title = match title {
            qdrant_client::qdrant::value::Kind::StringValue(value) => value,
            _ => panic!("value {:?} was not an integer", "title"),
        };
        let r#abstract = payload.get("abstract").cloned().unwrap().kind.unwrap();
        let r#abstract = match r#abstract {
            qdrant_client::qdrant::value::Kind::StringValue(value) => value,
            _ => panic!("value {:?} was not an integer", "abstract"),
        };
        let updated = payload.get("updated").cloned().unwrap().kind.unwrap();
        let updated = match updated {
            qdrant_client::qdrant::value::Kind::StringValue(value) => value,
            _ => panic!("value {:?} was not an integer", "updated"),
        };
        let updated = chrono::DateTime::parse_from_rfc3339(&updated).unwrap().with_timezone(&Utc);

        let openaccessinfo = payload.get("openaccessinfo").cloned().unwrap().kind.unwrap();
        let openaccessinfo = match openaccessinfo {
            qdrant_client::qdrant::value::Kind::StructValue(value) => value,
            _ => panic!("value {:?} was not an integer", "openaccessinfo"),
        };
        let external_ids = openaccessinfo.fields.get("externalids").cloned().unwrap().kind.unwrap();
        let external_ids = match external_ids {
            qdrant_client::qdrant::value::Kind::StructValue(value) => value,
            _ => panic!("error extracting external_ids"),
        };

        let openaccessinfo = OpenAccessInfo {
            externalids: Externalids {
                mag: external_ids.fields.get("mag").cloned().unwrap().kind.map(|s| match s {
                    qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                    _ => panic!("error extracting externalids::mag"),
                }),
                acl: external_ids.fields.get("acl").cloned().unwrap().kind.map(|s| match s {
                    qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                    _ => panic!("error extracting externalids::acl"),
                }),
                doi: external_ids.fields.get("doi").cloned().unwrap().kind.map(|s| match s {
                    qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                    _ => panic!("error extracting externalids::doi"),
                }),
                pubmedcentral: external_ids.fields.get("pubmedcentral").cloned().unwrap().kind.map(|s| match s {
                    qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                    _ => panic!("error extracting externalids::pubmedcentral"),
                }),
                arxiv: external_ids.fields.get("arxiv").cloned().unwrap().kind.map(|s| match s {
                    qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                    _ => panic!("error extracting externalids::arxiv"),
                }),
            },
            license: openaccessinfo.fields.get("license").cloned().unwrap().kind.map(|s| match s {
                qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                _ => panic!("error extracting openaccessinfo::license"),
            }),
            url: openaccessinfo.fields.get("url").cloned().unwrap().kind.map(|s| match s {
                qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                _ => panic!("error extracting openaccessinfo::url"),
            }),
            status: openaccessinfo.fields.get("status").cloned().unwrap().kind.map(|s| match s {
                qdrant_client::qdrant::value::Kind::StringValue(s) => s,
                _ => panic!("error extracting openaccessinfo::status"),
            }),
        };
        Paper {
            corpusid: corpusid as u64,
            title: title.to_string(),
            r#abstract,
            updated,
            openaccessinfo,
        }
    }
}

impl From<Paper> for qdrant_client::prelude::Payload {
    fn from(paper: Paper) -> Self {
        let mut payload: HashMap<&str, Value> = HashMap::new();
        payload.insert("corpusid", Value { kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(paper.corpusid as i64)) });
        payload.insert("title", Value { kind: Some(qdrant_client::qdrant::value::Kind::StringValue(paper.title)) });
        payload.insert("abstract", Value { kind: Some(qdrant_client::qdrant::value::Kind::StringValue(paper.r#abstract)) });
        payload.insert("updated", Value { kind: Some(qdrant_client::qdrant::value::Kind::StringValue(paper.updated.to_rfc3339())) });
        let mut openaccessinfo = HashMap::new();
        let mut externalids = HashMap::new();
        externalids.insert("mag".to_string(), Value { kind: paper.openaccessinfo.externalids.mag.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        externalids.insert("acl".to_string(), Value { kind: paper.openaccessinfo.externalids.acl.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        externalids.insert("doi".to_string(), Value { kind: paper.openaccessinfo.externalids.doi.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        externalids.insert("pubmedcentral".to_string(), Value { kind: paper.openaccessinfo.externalids.pubmedcentral.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        externalids.insert("arxiv".to_string(), Value { kind: paper.openaccessinfo.externalids.arxiv.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        openaccessinfo.insert("externalids".to_string(), Value { kind: Some(qdrant_client::qdrant::value::Kind::StructValue(qdrant_client::qdrant::Struct { fields: externalids })) });
        openaccessinfo.insert("license".to_string(), Value { kind: paper.openaccessinfo.license.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        openaccessinfo.insert("url".to_string(), Value { kind: paper.openaccessinfo.url.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        openaccessinfo.insert("status".to_string(), Value { kind: paper.openaccessinfo.status.map(|s| qdrant_client::qdrant::value::Kind::StringValue(s)) });
        payload.insert("openaccessinfo", Value { kind: Some(qdrant_client::qdrant::value::Kind::StructValue(qdrant_client::qdrant::Struct { fields: openaccessinfo })) });
        Payload::from(payload)
    }
}

impl From<surface::Paper> for Paper {
    fn from(paper: surface::Paper) -> Self {
        Paper {
            corpusid: paper.corpusid.parse().unwrap(),
            openaccessinfo: paper.openaccessinfo.unwrap().into(),
            title: paper.title,
            r#abstract: paper.r#abstract,
            updated: paper.updated.parse::<chrono::DateTime<chrono::Utc>>().unwrap(),
        }
    }
}

// impl From<qdrant_client::prelude::Payload> for Paper {
//     fn from(payload: Payload) -> Self {
//         let mut payload = payload;
//         let corpusid = extract_integer_value(&payload, "corpusid");
//         let title = extract_string_value(&payload, "title");
//         let r#abstract = extract_string_value(&payload, "abstract");
//         let updated = extract_string_value(&payload, "updated");
//         let updated = chrono::DateTime::parse_from_rfc3339(&updated).unwrap().with_timezone(&Utc);

//         let openaccessinfo = extract_struct_value(&payload, "openaccessinfo");
//         let external_ids = openaccessinfo.get("externalids").unwrap().kind.unwrap();
//         let external_ids = match external_ids {
//             qdrant_client::qdrant::value::Kind::StructValue(value) => value,
//             _ => panic!("error extracting external_ids"),
//         };

//         let openaccessinfo = OpenAccessInfo {
//             externalids: Externalids {
//                 mag: external_ids.fields.get("mag").unwrap().kind.map(|s| match s {
//                     qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                     _ => panic!("error extracting externalids::mag"),
//                 }),
//                 acl: external_ids.fields.get("acl").unwrap().kind.map(|s| match s {
//                     qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                     _ => panic!("error extracting externalids::acl"),
//                 }),
//                 doi: external_ids.fields.get("doi").unwrap().kind.map(|s| match s {
//                     qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                     _ => panic!("error extracting externalids::doi"),
//                 }),
//                 pubmedcentral: external_ids.fields.get("pubmedcentral").unwrap().kind.map(|s| match s {
//                     qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                     _ => panic!("error extracting externalids::pubmedcentral"),
//                 }),
//                 arxiv: external_ids.fields.get("arxiv").unwrap().kind.map(|s| match s {
//                     qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                     _ => panic!("error extracting externalids::arxiv"),
//                 }),
//             },
//             license: openaccessinfo.get("license").unwrap().kind.map(|s| match s {
//                 qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                 _ => panic!("error extracting license"),
//             }),
//             url: openaccessinfo.get("url").unwrap().kind.map(|s| match s {
//                 qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                 _ => panic!("error extracting url"),
//             }),
//             status: openaccessinfo.get("status").unwrap().kind.map(|s| match s {
//                 qdrant_client::qdrant::value::Kind::StringValue(s) => s,
//                 _ => panic!("error extracting status"),
//             }),
//         };
//         Paper {
//             corpusid: corpusid as u64,
//             openaccessinfo,
//             title,
//             r#abstract,
//             updated,
//         }
//     }
// }

use crate::surface;

impl From<surface::OpenAccessInfo> for OpenAccessInfo {
    fn from(oai: surface::OpenAccessInfo) -> Self {
        OpenAccessInfo {
            externalids: Externalids {
                mag: oai.externalids.clone().map(|e| e.mag),
                acl: oai.externalids.clone().map(|e| e.acl),
                doi: oai.externalids.clone().map(|e| e.doi),
                pubmedcentral: oai.externalids.clone().map(|e| e.pmid),
                arxiv: oai.externalids.clone().map(|e| e.arxiv),
            },
            license: Some(oai.license),
            url: Some(oai.url),
            status: Some(oai.status),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaperVec<T> {
    pub corpusid: u64,
    pub vector: T,
}