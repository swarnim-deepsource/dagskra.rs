use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer};

pub enum Status {
    Live,
    Repeat,
    Standard,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Listing {
    #[serde(rename = "startTime", deserialize_with = "de_datetime")]
    pub start_time: NaiveDateTime,
    pub title: String,
    description: String,
    live: bool,
}

impl Listing {
    fn is_repeat(&self) -> bool {
        self.description.trim().ends_with(" e.")
    }

    pub fn date(&self) -> String {
        self.start_time.format("%d.%m.%Y").to_string()
    }

    pub fn description(&self) -> &str {
        self.description.trim().trim_end_matches(" e.")
    }

    pub fn has_description(&self) -> bool {
        !self.description.trim().is_empty()
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
struct Response {
    results: Schedule,
}

pub async fn fetch_schedule() -> Result<Schedule, Box<dyn std::error::Error>> {
    let url = "https://apis.is/tv/ruv";
    let res: Response = reqwest::get(url).await?.json().await?;
    Ok(res.results)
}
