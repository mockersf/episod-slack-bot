use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryNotification {
    pub channel: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotification {
    pub link: String,
    pub message_ts: String,
    pub channel: String,
}
