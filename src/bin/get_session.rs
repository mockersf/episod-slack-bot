use std::env;

use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{error::HandlerError, lambda, Context};

mod common;

fn main() {
    lambda!(send_sessions)
}

fn send_sessions(notification: SnsEvent, _: Context) -> Result<(), HandlerError> {
    notification.records.iter().for_each(|notification| {
        let msg: common::GetNotification =
            serde_json::from_str(&notification.clone().sns.message.unwrap()).unwrap();

        let session = episod::extract_session_details(
            &msg.link,
            &msg.id,
            &reqwest::blocking::get(&msg.link).unwrap().text().unwrap(),
        );

        reqwest::blocking::Client::new()
            .post("https://slack.com/api/chat.unfurl")
            .json(&episod::slack::session_to_slack_unfurl(
                msg.channel,
                msg.message_ts,
                msg.link,
                session,
            ))
            .bearer_auth(env::var("slack_token").unwrap())
            .send()
            .unwrap()
            .text()
            .unwrap();
    });
    Ok(())
}
