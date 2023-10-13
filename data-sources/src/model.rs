use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "releaseDate", skip_deserializing)]
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub release_date: chrono::DateTime<Utc>,
    pub rating: Option<f32>,
    pub genres: Option<Vec<String>>,
    #[serde(rename = "imageUrl", skip_deserializing)]
    pub image_url: String,
    pub plot: Option<String>,
    pub rank: u32,
    #[serde(rename = "runningTimeSecs", skip_deserializing)]
    pub running_time_secs: u64,
    pub actors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mongo_uri: String,
}
