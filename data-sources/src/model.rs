use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => DateTime::parse_from_rfc3339(&s)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub year: u64,
    pub title: String,
    pub info: Info,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Info {
    pub directors: Option<Vec<String>>,
    #[serde(
        default,
        rename = "releaseDate",
        alias = "release_date",
        deserialize_with = "deserialize_date"
    )]
    pub release_date: Option<chrono::DateTime<Utc>>,
    pub rating: Option<f32>,
    pub genres: Option<Vec<String>>,
    #[serde(rename = "imageUrl", alias = "image_url")]
    pub image_url: Option<String>,
    pub plot: Option<String>,
    pub rank: u32,
    #[serde(rename = "runningTimeSecs", alias = "running_time_secs")]
    pub running_time_secs: Option<u64>,
    pub actors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mongo_uri: String,
}
