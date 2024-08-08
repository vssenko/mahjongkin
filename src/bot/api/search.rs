use mobot::API;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct SearchMessagesRequest {
    pub chat_id: i64,
    pub q: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchMessagesResponse {
    messages: Vec<mobot::api::Message>,
}

impl mobot::Request for SearchMessagesRequest {}

pub async fn search_messages(
    api: &API,
    r: SearchMessagesRequest,
) -> anyhow::Result<Vec<mobot::api::Message>> {
    log::info!("Loading messages...");

    //This does not work because Telegram Bot API cannot search for messages :-(

    let response = api
        .client
        .post::<SearchMessagesRequest, SearchMessagesResponse>("searchMessages", &r)
        .await?;

    log::info!("Loaded messages!");

    Ok(response.messages)
}
