use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Access {
    pub access_type: Option<String>,
    pub access_url: Option<String>,
    pub file_ident: Option<String>,
    pub mimetype: Option<String>,
    pub release_ident: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Abstract {
    pub body: String,
    pub lang_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Biblio {
    pub affiliations: Option<Vec<String>>,
    pub contrib_count: Option<i32>,
    pub contrib_names: Option<Vec<String>>,
    pub doi: Option<String>,
    pub doi_prefix: Option<String>,
    pub doi_registrar: Option<String>,
    pub issns: Option<Vec<String>>,
    pub lang_code: Option<String>,
    pub license_slug: Option<String>,
    pub publisher: Option<String>,
    pub release_date: Option<String>,
    pub release_ident: Option<String>,
    pub release_stage: Option<String>,
    pub release_type: Option<String>,
    pub release_year: Option<i32>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fulltext {
    pub file_mimetype: Option<String>,
    pub access_type: Option<String>,
    pub file_sha1: Option<String>,
    pub size_bytes: Option<i32>,
    pub lang_code: Option<String>,
    pub file_ident: Option<String>,
    pub access_url: Option<String>,
    pub release_ident: Option<String>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Release {
    pub arxiv_id: Option<String>,
    pub container_ident: Option<String>,
    pub container_issnl: Option<String>,
    pub container_name: Option<String>,
    pub doi: Option<String>,
    pub doi_prefix: Option<String>,
    pub doi_registrar: Option<String>,
    pub ident: Option<String>,
    pub release_date: Option<String>,
    pub release_stage: Option<String>,
    pub release_type: Option<String>,
    pub release_year: Option<i32>,
    pub revision: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub collapse_key: String,
    pub work_ident: String,
    pub access: Vec<Access>,
    pub abstracts: Vec<Abstract>,
    pub biblio: Biblio,
    pub fulltext: Fulltext,
    pub doc_type: String,
    pub doc_index_ts: String,
    pub key: String,
    pub releases: Vec<Release>,
    pub tags: Vec<String>,
    pub _highlights: Vec<String>,
    pub _collapsed: Vec<String>,
    pub _collapsed_count: i32,
}

use crate::surface;
use crate::surface::{Paper, OpenAccessInfo, Externalids};

impl From<&SearchResult> for Paper {
    fn from(sr: &SearchResult) -> Self {
        Paper {
            corpusid: sr.collapse_key.clone(),
            openaccessinfo: Some(OpenAccessInfo {
                externalids: Some(Externalids {
                    mag: "".to_string(),
                    acl: "".to_string(),
                    doi: sr.biblio.doi.clone().unwrap_or("".to_string()),
                    pmid: "".to_string(),
                    arxiv: sr.releases[0].arxiv_id.clone().unwrap_or("".to_string()),
                }),
                license: sr.biblio.license_slug.clone().unwrap_or("".to_string()),
                url: sr.fulltext.access_url.clone().unwrap_or("".to_string()),
                status: "".to_string(),
            }),
            title: sr.biblio.title.clone(),
            r#abstract: sr.abstracts.first().unwrap_or(&Abstract{body: "".to_string(), lang_code: None}).body.clone(),
            updated: chrono::Utc::now().to_string(),
            fulltext: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse {
    pub query_type: String,
    pub count_returned: i32,
    pub count_found: i32,
    pub offset: i32,
    pub limit: i32,
    pub deep_page_limit: i32,
    pub query_time_ms: i32,
    pub query_wall_time_ms: i32,
    pub results: Vec<SearchResult>,
}