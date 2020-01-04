use std::env;

use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_sns::Sns;

mod common;

fn main() {
    lambda!(send_sessions)
}

fn send_sessions(notification: SnsEvent, _: Context) -> Result<(), HandlerError> {
    notification.records.iter().for_each(|notification| {
        let msg: common::QueryNotification =
            serde_json::from_str(&notification.clone().sns.message.unwrap()).unwrap();

        let url = format!("https://www.episod.com/wp-admin/admin-ajax.php?page={}&wctrl=session&waction=ajax_get_sessions&wplug=wr", msg.page);

        let planning = reqwest::blocking::get(&url)
                .unwrap()
                .text()
                .unwrap();

        let sessions: Vec<_> = episod::extract_sessions_with_filter(
            &planning,
            &episod::filters::Filters::from_query(&msg.query),
        ).into_iter().filter(|session| !msg.found.contains(&session.id)).collect();

        let channel = msg.channel.clone();

        reqwest::blocking::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .json(&episod::slack::sessions_to_slack_message(
                &sessions, channel,
            ))
            .bearer_auth(env::var("slack_token").unwrap())
            .send()
            .unwrap()
            .text()
            .unwrap();

        if msg.found.len() + sessions.len() < 5 && msg.page < 2 {
            let next = common::QueryNotification {
                        page: msg.page + 1,
                        found: [
                            &msg.found[..],
                            &sessions.iter().map(|session| session.id.clone()).collect::<Vec<_>>()[..]
                        ].concat(),
                        ..msg
                    };
            let client = rusoto_sns::SnsClient::new(rusoto_core::Region::UsEast1);
            client
                .publish(rusoto_sns::PublishInput {
                    message: serde_json::to_string(&next).unwrap(),
                    topic_arn: Some(env::var("topic_session_query").unwrap()),
                    ..Default::default()
                })
                .sync()
                .unwrap();
        }
    });
    Ok(())
}
