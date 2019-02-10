#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

mod aws_helpers;

lazy_static! {
    pub static ref PLANNING_HTML: String = {
        reqwest::get("https://www.episod.com/planning/")
            .unwrap()
            .text()
            .unwrap()
    };
}

use std::env;

fn send_sessions(notifications: &aws_lambda::event::sns::SnsEvent) {
    notifications.records.iter().for_each(|notification| {
        let msg: aws_helpers::Notification =
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
}

fn main() {
    aws_lambda::start(|notifications: aws_lambda::event::sns::SnsEvent| {
        send_sessions(&notifications);
        Ok(())
    })
}
