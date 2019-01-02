use rand::seq::SliceRandom;
use rand::thread_rng;
use slack_push::message::{Attachment, AttachmentAction, Message};

use super::Session;

static COLORS: [&'static str; 13] = [
    "#C0C0C0", "#FF0000", "#00FF00", "#439FE0", "#00FFFF", "#008080", "#0000FF", "#FF00FF",
    "#800080", "#3cb371", "#ffa500", "#6a5acd", "#ee82ee",
];

pub fn sessions_to_slack_message(sessions: &[Session], channel: String) -> Message {
    let mut rng = thread_rng();

    Message {
            attachments: Some(sessions
                .iter()
                .take(10)
                .map(|session| Attachment {
                    text: Some(format!(
                        "{} le *{}* à *{}* ({} minutes)",
                        session.sport, session.date, session.time, session.duration_minutes
                    )),
                    actions: Some(vec![AttachmentAction::Button {
                        url: Some(session.reservation_link.clone()),
                        text: "Réserver 🏅".to_string(),
                        style: Some("primary".to_string()),
                        name: None,
                        value: None,
                        confirm: None,
                    }]),
                    thumb_url: match session.sport.as_ref() {
                        "bootcamp" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/weight-lifter_1f3cb.png".to_string()),
                        "boxing" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/boxing-glove_1f94a.png".to_string()),
                        "yoga-vinyasa" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "yin-yoga" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "yoga-hatha" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-in-lotus-position_1f9d8.png".to_string()),
                        "rowing" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/rowboat_1f6a3.png".to_string()),
                        "cycling" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/bicyclist_1f6b4.png".to_string()),
                        "pilates" => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/person-doing-cartwheel_1f938.png".to_string()),
                        _ => Some("https://emojipedia-us.s3.dualstack.us-west-1.amazonaws.com/thumbs/240/apple/129/flexed-biceps_1f4aa.png".to_string()),
                    },
                    color: Some(COLORS.choose(&mut rng).unwrap().to_string()),
                    author_name: Some(format!("{} ({})", session.coach, session.hub)),
                    ..Default::default()
                }).collect()),
            channel,
            ..Default::default()
        }
}
