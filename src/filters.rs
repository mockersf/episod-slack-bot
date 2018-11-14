use chrono::prelude::*;

#[derive(Debug, Deserialize, Default)]
pub struct Filters {
    pub hub: Option<String>,
    pub coach: Option<String>,
    pub sport: Option<String>,
    pub day: Option<Day>,
    pub date: Option<NaiveDate>,
    pub period: Option<Period>,
}
impl Filters {
    fn sport_from_query(query: &str) -> Option<String> {
        match query {
            query if query.contains("boxe") => Some("boxing".to_string()),
            query if query.contains("boxing") => Some("boxing".to_string()),
            query if query.contains("yoga") => Some("yoga".to_string()),
            query if query.contains("bootcamp") => Some("bootcamp".to_string()),
            query if query.contains("pilates") => Some("pilates".to_string()),
            _ => None,
        }
    }

    fn hub_from_query(query: &str) -> Option<String> {
        match query {
            query if query.contains("bourse") => Some("bourse".to_string()),
            query if query.contains("republique") => Some("republique".to_string()),
            query if query.contains("république") => Some("republique".to_string()),
            query if query.contains("clichy") => Some("clichy".to_string()),
            _ => None,
        }
    }

    fn coach_from_query(query: &str) -> Option<String> {
        match query {
            query if query.contains("alexandre") => Some("alexandre".to_string()),
            query if query.contains("alyona") => Some("alyona".to_string()),
            query if query.contains("amadou") => Some("amadou".to_string()),
            query if query.contains("ange") => Some("ange".to_string()),
            query if query.contains("anna") => Some("anna".to_string()),
            query if query.contains("anne-julie") => Some("anne-julie".to_string()),
            query if query.contains("avine") => Some("avine".to_string()),
            query if query.contains("bouba") => Some("bouba".to_string()),
            query if query.contains("clelia") => Some("clelia".to_string()),
            query if query.contains("clio-pajczer") => Some("clio-pajczer".to_string()),
            query if query.contains("djamel") => Some("djamel".to_string()),
            query if query.contains("emelyne") => Some("emelyne".to_string()),
            query if query.contains("emeric") => Some("emeric".to_string()),
            query if query.contains("estelle") => Some("estelle".to_string()),
            query if query.contains("evidant") => Some("evidant".to_string()),
            query if query.contains("floriane") => Some("floriane".to_string()),
            query if query.contains("franck") => Some("franck".to_string()),
            query if query.contains("gary") => Some("gary".to_string()),
            query if query.contains("guillaume") => Some("guillaume".to_string()),
            query if query.contains("helen") => Some("helen".to_string()),
            query if query.contains("jimmy") => Some("jimmy".to_string()),
            query if query.contains("john") => Some("john".to_string()),
            query if query.contains("johnny") => Some("johnny".to_string()),
            query if query.contains("karine") => Some("karine".to_string()),
            query if query.contains("laura") => Some("laura".to_string()),
            query if query.contains("laurence") => Some("laurence".to_string()),
            query if query.contains("ludo") => Some("ludo".to_string()),
            query if query.contains("ludovic") => Some("ludo".to_string()),
            query if query.contains("ludob") => Some("ludob".to_string()),
            query if query.contains("ludo-m") => Some("ludo-m".to_string()),
            query if query.contains("ludovic-r") => Some("ludovic-r".to_string()),
            query if query.contains("lylou") => Some("lylou".to_string()),
            query if query.contains("marie-soline") => Some("marie-soline".to_string()),
            query if query.contains("martin") => Some("martin".to_string()),
            query if query.contains("maxime") => Some("maxime".to_string()),
            query if query.contains("michael") => Some("michael".to_string()),
            query if query.contains("minh") => Some("minh".to_string()),
            query if query.contains("nasser") => Some("nasser".to_string()),
            query if query.contains("nhick") => Some("nhick".to_string()),
            query if query.contains("olivia") => Some("olivia".to_string()),
            query if query.contains("pascale") => Some("pascale".to_string()),
            query if query.contains("patrice") => Some("patrice".to_string()),
            query if query.contains("philippe") => Some("philippe".to_string()),
            query if query.contains("rachid") => Some("rachid".to_string()),
            query if query.contains("rahim") => Some("rahim".to_string()),
            query if query.contains("rakesh") => Some("rakesh".to_string()),
            query if query.contains("salim") => Some("salim".to_string()),
            query if query.contains("skander") => Some("skander".to_string()),
            query if query.contains("sophia") => Some("sophia".to_string()),
            query if query.contains("souleymane") => Some("souleymane".to_string()),
            query if query.contains("steven") => Some("steven".to_string()),
            query if query.contains("ted") => Some("ted".to_string()),
            query if query.contains("thierry") => Some("thierry".to_string()),
            query if query.contains("yassin") => Some("yassin".to_string()),
            _ => None,
        }
    }

    fn period_from_query(query: &str) -> Option<Period> {
        match query {
            query if query.contains("matin") => Some(Period::Matin),
            query if query.contains("midi") => Some(Period::Midi),
            query if query.contains("soir") => Some(Period::Soir),
            _ => None,
        }
    }

    fn day_from_query(query: &str) -> Option<Day> {
        match query {
            query if query.contains("lundi") => Some(Day::Lundi),
            query if query.contains("mardi") => Some(Day::Mardi),
            query if query.contains("mercredi") => Some(Day::Mercredi),
            query if query.contains("jeudi") => Some(Day::Jeudi),
            query if query.contains("vendredi") => Some(Day::Vendredi),
            query if query.contains("samedi") => Some(Day::Samedi),
            query if query.contains("dimanche") => Some(Day::Dimanche),
            query if query.contains("demain") => Some(Day::Demain),
            query if query.contains("aujourd'hui") => Some(Day::Today),
            query if query.contains(" ce ") => Some(Day::Today),
            _ => None,
        }
    }

    fn date_from_query(query: &str) -> Option<NaiveDate> {
        query
            .split(' ')
            .filter(|t| t.contains('/'))
            .last()
            .and_then(super::helpers::short_date_to_date)
    }

    pub fn from_query(query: &str) -> Self {
        Filters {
            sport: Self::sport_from_query(query),
            coach: Self::coach_from_query(query),
            date: Self::date_from_query(query),
            day: Self::day_from_query(query),
            hub: Self::hub_from_query(query),
            period: Self::period_from_query(query),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Day {
    Lundi,
    Mardi,
    Mercredi,
    Jeudi,
    Vendredi,
    Samedi,
    Dimanche,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Demain,
    Tomorrow,
    Today,
}
impl Day {
    pub fn to_weekday(&self) -> Weekday {
        match self {
            Day::Lundi | Day::Monday => Weekday::Mon,
            Day::Mardi | Day::Tuesday => Weekday::Tue,
            Day::Mercredi | Day::Wednesday => Weekday::Wed,
            Day::Jeudi | Day::Thursday => Weekday::Thu,
            Day::Vendredi | Day::Friday => Weekday::Fri,
            Day::Samedi | Day::Saturday => Weekday::Sat,
            Day::Dimanche | Day::Sunday => Weekday::Sun,
            Day::Demain | Day::Tomorrow => Utc::now().naive_utc().weekday().succ(),
            Day::Today => Utc::now().naive_utc().weekday(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Matin,
    Midi,
    Soir,
    Morning,
    Lunch,
    Evening,
}
impl Period {
    pub fn match_time(&self, time: NaiveTime) -> bool {
        match self {
            Period::Matin | Period::Morning => time < NaiveTime::from_hms(11, 0, 0),
            Period::Soir | Period::Evening => time > NaiveTime::from_hms(17, 0, 0),
            Period::Midi | Period::Lunch => {
                time > NaiveTime::from_hms(11, 45, 0) && time < NaiveTime::from_hms(14, 15, 0)
            }
        }
    }
}
