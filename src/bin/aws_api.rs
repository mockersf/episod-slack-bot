#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate http;

#[macro_use]
extern crate failure;

extern crate aws_lambda;
extern crate rusoto_core;
extern crate rusoto_sns;

extern crate slack_push;

extern crate episod;

use std::collections::HashMap;
use std::env;

use rusoto_sns::Sns;
use slack_push::{Event, EventInfo};

mod aws_api_helpers;
mod aws_helpers;

pub fn slack_event(
    req: &aws_api_helpers::ShortApiGatewayProxyRequest,
) -> Result<http::response::Response<String>, failure::Error> {
    let event: Event = serde_json::from_str(&req.clone().body.unwrap())?;
    match event {
        Event::UrlVerification { challenge, .. } => {
            Ok(http::response::Builder::new().status(200).body(challenge)?)
        }
        Event::EventCallback { event, token, .. } => match event {
            EventInfo::AppMention { channel, text, .. } => {
                let client = rusoto_sns::SnsClient::new(rusoto_core::Region::UsEast1);
                client
                    .publish(rusoto_sns::PublishInput {
                        message: serde_json::to_string(&aws_helpers::Notification {
                            token,
                            channel,
                            query: text,
                        })
                        .unwrap(),
                        topic_arn: Some(env::var("topic").unwrap()),
                        ..Default::default()
                    })
                    .sync()
                    .unwrap();
                Ok(http::response::Builder::new()
                    .status(200)
                    .body("".to_string())?)
            }
            _ => Ok(http::response::Builder::new()
                .status(400)
                .body("unsupported event type".to_string())?),
        },
        _ => Ok(http::response::Builder::new()
            .status(400)
            .body("unsupported event type".to_string())?),
    }
}

fn main() {
    aws_lambda::start(|req: aws_api_helpers::ShortApiGatewayProxyRequest| {
        let response = match (req.http_method.as_ref(), req.path.as_ref()) {
            ("POST", "/slack-event") => slack_event(&req),
            (method, path) => Err(aws_api_helpers::HttpError::UnexpectedPath {
                method: method.to_string(),
                path: path.to_string(),
            }
            .into()),
        };

        Ok(match response {
            Ok(response) => {
                let a: aws_api_helpers::Response = response.into();
                a.0
            }
            Err(err) => aws_lambda::event::apigw::ApiGatewayProxyResponse {
                body: Some(format!("{}", err)),
                status_code: 500,
                is_base64_encoded: None,
                headers: HashMap::new(),
            },
        })
    })
}
