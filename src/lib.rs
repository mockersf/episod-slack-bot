extern crate chrono;
extern crate rand;
extern crate select;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

extern crate slack_push;

mod extract;
pub use extract::extract_sessions_with_filter;
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
