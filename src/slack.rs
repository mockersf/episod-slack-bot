use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use slack_push::message::{Attachment, AttachmentAction, Message};

use super::Session;

static COLORS: [&str; 13] = [
    "#C0C0C0", "#FF0000", "#00FF00", "#439FE0", "#00FFFF", "#008080", "#0000FF", "#FF00FF",
    "#800080", "#3cb371", "#ffa500", "#6a5acd", "#ee82ee",
];

fn emoji_to_url(emoji: &str) -> String {
    format!(
        "https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/{}.png",
        emoji
    )
}

fn session_to_attachement(session: &Session) -> Attachment {
    let mut rng = thread_rng();

    Attachment {
        text: Some(format!(
            "{} le *{}* à *{}* ({} minutes)",
            session.sport, session.date, session.time, session.duration_minutes
        )),
        actions: Some(vec![AttachmentAction::Button {
            url: Some(session.reservation_link.clone()),
            text: if session.full {
                "Complet 🤷‍".to_string()
            } else {
                "Réserver 🏅".to_string()
            },
            style: Some("primary".to_string()),
            name: None,
            value: None,
            confirm: None,
        }]),
        thumb_url: match session.sport.as_ref() {
            "bootcamp" => Some(emoji_to_url("weight-lifter_1f3cb")),
            "boxing" => Some(emoji_to_url("boxing-glove_1f94a")),
            "yoga-vinyasa" => Some(emoji_to_url("person-in-lotus-position_1f9d8")),
            "yin-yoga" => Some(emoji_to_url("person-in-lotus-position_1f9d8")),
            "yoga-hatha" => Some(emoji_to_url("person-in-lotus-position_1f9d8")),
            "rowing" => Some(emoji_to_url("rowboat_1f6a3")),
            "cycling" => Some(emoji_to_url("bicyclist_1f6b4")),
            "pilates" => Some(emoji_to_url("person-doing-cartwheel_1f938")),
            _ => Some(emoji_to_url("flexed-biceps_1f4aa")),
        },
        color: Some((*COLORS.choose(&mut rng).unwrap()).to_string()),
        author_name: Some(format!("{} ({})", session.coach, session.hub)),
        ..Default::default()
    }
}

pub fn sessions_to_slack_message(sessions: &[Session], channel: String) -> Message {
    Message {
        attachments: Some(
            sessions
                .iter()
                .take(10)
                .map(|session| session_to_attachement(session))
                .collect(),
        ),
        channel: Some(channel),
        ..Default::default()
    }
}

#[derive(Debug, Serialize)]
pub struct Unfurled {
    channel: String,
    ts: String,
    unfurls: HashMap<String, Attachment>,
}

pub fn session_to_slack_unfurl(
    channel: String,
    message_ts: String,
    link: String,
    session: Session,
) -> Unfurled {
    let mut links = HashMap::new();
    links.insert(link, session_to_attachement(&session));
    Unfurled {
        channel,
        ts: message_ts,
        unfurls: links,
    }
}
