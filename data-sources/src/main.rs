pub mod model;

use model::Movie;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let json_file_path = Path::new("data.json");
    let mut file = File::open(json_file_path).expect("Unable to open json file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let movies: Vec<Movie> = serde_json::from_str(&contents).expect("Unable to parse JSON");
    for movie in movies {
        println!("{:#?}", movie);
    }
}
