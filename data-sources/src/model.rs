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
    pub directors: Vec<String>,
    #[serde(rename = "releaseDate")]
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub release_date: chrono::DateTime<Utc>,
    pub rating: f32,
    pub genres: Vec<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub plot: String,
    pub rank: u32,
    #[serde(rename = "runningTimeSecs")]
    pub running_time_secs: u64,
    pub actors: Vec<String>,
}
