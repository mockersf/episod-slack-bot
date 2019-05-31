use serde::Serialize;

mod extract;
pub use crate::extract::{extract_session_details, extract_sessions_with_filter};
pub mod filters;
mod helpers;
pub mod slack;

#[derive(Debug, Serialize, Clone)]
pub struct Session {
    pub reservation_link: String,
    pub coach: String,
    pub hub: String,
    pub sport: String,
    pub duration_minutes: i64,
    pub full: bool,
    pub time: chrono::NaiveTime,
    pub date: chrono::NaiveDate,
}
