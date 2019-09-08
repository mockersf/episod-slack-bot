use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryNotification {
    pub channel: String,
    pub query: String,
    pub page: usize,
    pub found: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotification {
    pub link: String,
    pub id: String,
    pub message_ts: String,
    pub channel: String,
}
