use std::env;

use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{error::HandlerError, lambda, Context};
use lazy_static::lazy_static;

mod common;

lazy_static! {
    pub static ref PLANNING_HTML: String = {
        reqwest::get("https://www.episod.com/planning/")
            .unwrap()
            .text()
            .unwrap()
    };
}

fn main() {
    lambda!(send_sessions)
}

fn send_sessions(notification: SnsEvent, _: Context) -> Result<(), HandlerError> {
    notification.records.iter().for_each(|notification| {
        let msg: common::QueryNotification =
            serde_json::from_str(&notification.clone().sns.message.unwrap()).unwrap();

        let sessions = episod::extract_sessions_with_filter(
            &PLANNING_HTML,
            &episod::filters::Filters::from_query(&msg.query),
        );

        reqwest::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .json(&episod::slack::sessions_to_slack_message(
                &sessions,
                msg.channel,
            ))
            .bearer_auth(env::var("slack_token").unwrap())
            .send()
            .unwrap()
            .text()
            .unwrap();
    });
    Ok(())
}
