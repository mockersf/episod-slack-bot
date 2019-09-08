use std::env;

use failure::{Error, Fail};
use lambda_http::{http::StatusCode, lambda, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use regex::Regex;
use rusoto_sns::Sns;
use slack_push::{Event, EventInfo};

mod common;

fn main() {
    lambda!(handler)
}

#[derive(Debug, Fail)]
pub enum HttpError {
    #[fail(display = "unexpected query: {} {}", method, path)]
    UnexpectedQuery { method: String, path: String },
}

fn handler(request: Request, _context: Context) -> Result<impl IntoResponse, HandlerError> {
    match (request.method().as_ref(), request.uri().path()) {
        ("POST", "/slack-event") => slack_event(&request).map_err(std::convert::Into::into),
        (method, path) => {
            let e = HttpError::UnexpectedQuery {
                method: method.to_string(),
                path: path.to_string(),
            };
            Err(Error::from(e))?
        }
    }
}

pub fn slack_event(req: &Request) -> Result<Response<String>, failure::Error> {
    let event: Event = serde_json::from_slice(req.body().as_ref())?;
    match event {
        Event::UrlVerification { challenge, .. } => {
            Ok(Response::builder().status(StatusCode::OK).body(challenge)?)
        }
        Event::EventCallback { event, .. } => match event {
            EventInfo::AppMention { channel, text, .. } => {
                let client = rusoto_sns::SnsClient::new(rusoto_core::Region::UsEast1);
                client
                    .publish(rusoto_sns::PublishInput {
                        message: serde_json::to_string(&common::QueryNotification {
                            channel,
                            query: text,
                            page: 1,
                            found: vec![],
                        })
                        .unwrap(),
                        topic_arn: Some(env::var("topic_session_query").unwrap()),
                        ..Default::default()
                    })
                    .sync()
                    .unwrap();
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body("".to_string())?)
            }
            EventInfo::LinkShared {
                channel,
                links,
                message_ts,
                ..
            } => {
                let episod_session_link =
                    Regex::new(r"https?://www.episod.com/reservation/(?P<session_id>\d+)/?$")
                        .unwrap();
                let client = rusoto_sns::SnsClient::new(rusoto_core::Region::UsEast1);
                links
                    .into_iter()
                    .map(|link| link.url)
                    .filter_map(|url| {
                        episod_session_link.captures(&url).map(|captures| {
                            (
                                url.clone(),
                                String::from(
                                    captures
                                        .name("session_id")
                                        .expect("session id not found in link but was captured")
                                        .as_str(),
                                ),
                            )
                        })
                    })
                    .map(|(url, session_id)| common::GetNotification {
                        link: url,
                        id: session_id,
                        channel: channel.clone(),
                        message_ts: message_ts.clone(),
                    })
                    .for_each(|notif| {
                        client
                            .publish(rusoto_sns::PublishInput {
                                message: serde_json::to_string(&notif).unwrap(),
                                topic_arn: Some(env::var("topic_session_get").unwrap()),
                                ..Default::default()
                            })
                            .sync()
                            .unwrap();
                    });
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body("".to_string())?)
            }
            _ => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("unsupported event type".to_string())?),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("unsupported event type".to_string())?),
    }
}
