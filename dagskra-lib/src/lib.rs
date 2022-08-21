use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer};
use serde_with::{serde_as, NoneAsEmptyString};

pub enum Status {
    Live,
    Repeat,
    Standard,
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Listing {
    pub title: String,
    #[serde(deserialize_with = "de_datetime", rename = "startTime")]
    start_time: NaiveDateTime,
    #[serde_as(as = "NoneAsEmptyString")]
    description: Option<String>,
    live: bool,
}

impl Listing {
    fn is_repeat(&self) -> bool {
        self.description
            .as_ref()
            .map_or(false, |s| s.trim().ends_with(" e."))
    }

    pub fn date(&self) -> String {
        self.start_time.format("%d.%m.%Y").to_string()
    }

    pub fn description(&self) -> &str {
        self.description
            .as_ref()
            .map_or("", |s| s.trim().trim_end_matches(" e."))
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    pub fn status(&self) -> Status {
        match (self.live, self.is_repeat()) {
            (true, _) => Status::Live,
            (false, true) => Status::Repeat,
            _ => Status::Standard,
        }
    }

    pub fn time(&self) -> String {
        self.start_time.format("%H:%M").to_string()
    }
}

fn de_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let fmt = "%Y-%m-%d %H:%M:%S";
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, fmt).map_err(D::Error::custom)?;
    Ok(dt)
}

pub type Schedule = Vec<Listing>;

#[derive(Deserialize)]
struct APIResponse {
    results: Schedule,
}

pub async fn fetch_schedule() -> Result<Schedule, Box<dyn std::error::Error>> {
    let url = "https://apis.is/tv/ruv";
    let res: APIResponse = reqwest::get(url).await?.json().await?;
    Ok(res.results)
}
