use chrono::{prelude::*, Duration};

pub fn short_date_to_date(short_date: &str) -> Option<NaiveDate> {
    let now = Utc::now().naive_utc();
    let year = now.year();

    let date_components: Vec<&str> = short_date.split('/').collect();
    if date_components.len() != 2 {
        return None;
    };
    let day = date_components.get(0).and_then(|v| v.parse::<u32>().ok())?;
    let month = date_components.get(1).and_then(|v| v.parse::<u32>().ok())?;
    let date = NaiveDate::from_ymd(year, month, day);
    if date < now.date() {
        Some(NaiveDate::from_ymd(year + 1, month, day))
    } else {
        Some(date)
    }
}

pub fn time_to_time(time: &str) -> NaiveTime {
    let time_components: Vec<&str> = time.split('H').collect();
    NaiveTime::from_hms(
        time_components[0].parse::<u32>().unwrap(),
        time_components[1].parse::<u32>().unwrap(),
        0,
    )
}

pub fn duration_to_duration(duration: &str, split: char) -> i64 {
    let duration_components: Vec<&str> = duration.split(split).collect();
    match duration_components.get(1).copied() {
        Some("mins") => Duration::minutes(duration_components[0].parse::<i64>().unwrap()),
        _ => Duration::minutes(duration_components[0].parse::<i64>().unwrap()),
    }
    .num_minutes()
}
