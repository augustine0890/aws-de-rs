use model::Config;
use mongodb::{error::Error, options::ClientOptions, Client, Database};
use serde_yaml;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::model::{self, Movie};

pub fn read_config() -> Config {
    let mut file = File::open(Path::new("config.yaml")).expect("Couldn't open config.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let config: Config = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
    config
}

pub async fn connect_mongo() -> Result<Database, Error> {
    let config = read_config();
    let client_options = ClientOptions::parse(config.mongo_uri)
        .await
        .expect("Failed to parse uri");
    let client =
        Client::with_options(client_options).expect("Failed to initialize standalone client");
    Ok(client.database("test"))
}

pub async fn insert_many_movies(db: &Database, movies: Vec<Movie>) -> Result<(), Error> {
    let collection = db.collection::<mongodb::bson::Document>("movies");
    // Convert Vec<Movie> into Vec<Document>
    let documents: Vec<bson::Document> = movies
        .into_iter()
        .map(|movie| {
            bson::to_bson(&movie)
                .unwrap()
                .as_document()
                .unwrap()
                .clone()
        })
        .collect();

    // Insert documents into the collection
    collection.insert_many(documents, None).await?;
    Ok(())
}

// pub async fn insert_many(db: &Database, movies: Vec<Movie>) -> Result<(), Error> {
// let collection = db.collection::<mongodb::bson::Document>("movies");
// // Convert Vec<Movie> into Vec<Document>
// let documents: Vec<bson::Document> = movies
// .into_iter()
// .map(|movie| {
// match bson::to_bson(&movie).unwrap() {
// bson::Bson::Document(document) => document,
// _ => panic!("Expected a Document"),
// }
// })
// .collect();

// // Insert documents into the collection
// collection.insert_many(documents, None).await?;
// Ok(())
// }
