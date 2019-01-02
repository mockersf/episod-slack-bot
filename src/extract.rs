use std::sync::{Arc, RwLock};

use chrono::prelude::*;

use select::predicate::{Attr, Class, Name, Not, Predicate};
use select::{document::Document, node::Node};

use super::{filters::Filters, Session};

lazy_static! {
    pub static ref CACHE_SESSIONS: Arc<RwLock<Option<Vec<Session>>>> =
        { Arc::new(RwLock::new(None)) };
}

pub fn extract_sessions_with_filter<'a>(html: &'a str, filters: &Filters) -> Vec<Session> {
    let mut sessions_lock = CACHE_SESSIONS.write().unwrap();
    let sessions = if let Some(sessions) = sessions_lock.clone() {
        sessions
    } else {
        let sessions: Vec<Session> = Document::from(html)
            .find(
                Name("li")
                    .and(Class("planning-item"))
                    .and(Not(Attr("id", "no-session"))),
            )
            .map(node_to_session)
            .collect();
        *sessions_lock = Some(sessions.clone());
        sessions
    };

    sessions
        .iter()
        .filter(|session| match filters.hub {
            Some(ref hub) => session.hub.contains(hub),
            _ => true,
        })
        .filter(|session| match filters.coach {
            Some(ref coach) => session.coach.contains(coach),
            _ => true,
        })
        .filter(|session| match filters.sport {
            Some(ref sport) => session.sport.contains(sport),
            _ => true,
        })
        .filter(|session| match filters.day {
            Some(ref day) => session.date.weekday() == day.to_weekday(),
            _ => true,
        })
        .filter(|session| match filters.date {
            Some(ref date) => session.date == *date,
            _ => true,
        })
        .filter(|session| match filters.period {
            Some(ref period) => period.match_time(session.time),
            _ => true,
        })
        .cloned()
        .collect()
}

pub fn node_to_session(node: Node) -> super::Session {
    super::Session {
        reservation_link: node.attr("data-href").unwrap().to_string(),
        sport: node.attr("data-sport").unwrap().to_string(),
        coach: node.attr("data-coach").unwrap().to_string(),
        hub: node.attr("data-hub").unwrap().to_string(),
        full: node.is(Class("status-complet")),
        duration_minutes: ::helpers::duration_to_duration(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("span"))
                .last()
                .unwrap()
                .text(),
        ),
        time: ::helpers::time_to_time(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("time"))
                .last()
                .unwrap()
                .text(),
        ),
        date: ::helpers::short_date_to_date(
            &node
                .find(Class("planning-date"))
                .last()
                .unwrap()
                .find(Name("div"))
                .last()
                .unwrap()
                .text(),
        )
        .unwrap(),
    }
}
