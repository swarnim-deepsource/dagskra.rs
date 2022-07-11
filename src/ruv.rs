use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer};

pub enum Status {
    Live,
    Repeat,
    Standard,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Show {
    #[serde(rename = "startTime", deserialize_with = "de_datetime")]
    start_time: NaiveDateTime,
    title: String,
    description: String,
    live: bool,
}

impl Show {
    fn is_repeat(&self) -> bool {
        self.description.trim().ends_with(" e.")
    }

    pub fn get_description(&self) -> &str {
        self.description.trim().trim_end_matches(" e.")
    }

    pub fn get_start_time(&self) -> String {
        self.start_time.format("%H:%M").to_string()
    }

    pub fn get_status(&self) -> Status {
        match (self.live, self.is_repeat()) {
            (true, _) => Status::Live,
            (false, true) => Status::Repeat,
            _ => Status::Standard,
        }
    }

    pub fn get_title(&self) -> &str {
        self.title.trim()
    }

    pub fn has_description(&self) -> bool {
        !self.description.trim().is_empty()
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

pub type Shows = Vec<Show>;

#[derive(Deserialize)]
struct Response {
    results: Shows,
}

pub async fn get_shows() -> Result<Shows, Box<dyn std::error::Error>> {
    let url = "https://apis.is/tv/ruv";
    tracing::debug!("Fetching schedule data from {}", url);
    let res: Response = reqwest::get(url).await?.json().await?;
    Ok(res.results)
}
