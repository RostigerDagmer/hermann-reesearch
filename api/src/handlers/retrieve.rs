use tokenizers::tokenizer::{Result, Tokenizer};
use warp::*;
use serde::{Deserialize, Serialize};


async fn __get_similar_str(query: &str, mut manager: DBAccessManager) -> Result<impl Reply, Rejection> {
    manager.get_similar_str(query).await?;
    respond(id_response, warp::http::StatusCode::OK)
}

pub fn get_similar_str(context: crate::manager::Context) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("query" / str )                    // Match /users/{id} path
        .and(warp::get())                  // Match GET method
        .and(super::with_db_access_manager(pool))  // Add DBAccessManager to params tuple
        .and_then(__get_video_captions)            // Pass the params touple to the handler function
}