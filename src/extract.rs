use std::sync::{Arc, RwLock};

use chrono::prelude::*;
use lazy_static::lazy_static;

use select::predicate::{Attr, Class, Name, Not, Predicate};
use select::{document::Document, node::Node};

use super::{filters::Filters, Session};

lazy_static! {
    pub static ref CACHE_SESSIONS: Arc<RwLock<Option<Vec<Session>>>> =
        { Arc::new(RwLock::new(None)) };
}

pub fn extract_session_details<'a>(link: &str, html: &'a str) -> Session {
    Document::from(html)
        .find(Name("div").and(Class("session-item")))
        .next()
        .map(|node| super::Session {
            reservation_link: String::from(link),
            coach: node
                .find(Name("span"))
                .last()
                .expect("no coach")
                .text()
                .split_off(5),
            date: crate::helpers::short_date_to_date(
                &node
                    .find(Name("h4").and(Class("masterclass-txt")))
                    .last()
                    .expect("no date")
                    .text()
                    .split(' ')
                    .last()
                    .expect("no ' '"),
            )
            .unwrap(),
            time: crate::helpers::time_to_time(
                node.find(Name("time"))
                    .last()
                    .expect("could not find time")
                    .text()
                    .split(" (")
                    .next()
                    .expect("no ' ('"),
            ),
            duration_minutes: crate::helpers::duration_to_duration(
                node.find(Name("time"))
                    .last()
                    .expect("could not find time")
                    .text()
                    .split('(')
                    .last()
                    .expect("no (")
                    .split(')')
                    .next()
                    .expect("no )"),
                ' ',
            ),
            full: false,
            hub: node
                .find(Name("address"))
                .last()
                .expect("address not found")
                .first_child()
                .expect("got a first child")
                .text(),
            sport: node
                .find(Name("h2").and(Class("masterclass-txt")))
                .last()
                .expect("sport not found")
                .text()
                .to_lowercase(),
        })
        .expect("no node found for session")
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
        .filter(|session| {
            session
                .date
                .signed_duration_since(Utc::now().naive_utc().date())
                .num_days()
                < 8
        })
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
        duration_minutes: crate::helpers::duration_to_duration(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("span"))
                .last()
                .unwrap()
                .text(),
            '\u{a0}',
        ),
        time: crate::helpers::time_to_time(
            &node
                .find(Class("planning-time"))
                .last()
                .unwrap()
                .find(Name("time"))
                .last()
                .unwrap()
                .text(),
        ),
        date: crate::helpers::short_date_to_date(
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
