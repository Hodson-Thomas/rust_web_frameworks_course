use serde::{Deserialize, Serialize};
use tide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    source: String,
    content: String,
}

pub async fn handle(mut req: tide::Request<()>) -> tide::Result {
    let user_data: Message = req.body_json().await?;
    if user_data.source.is_empty() || user_data.content.is_empty() {
        return Ok(tide::Response::new(tide::StatusCode::BadRequest));
    }
    Ok(json!({
        "source": "I'm a source",
        "message": "I'm a message"
    }).into())
}
