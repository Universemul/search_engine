use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use serde::de;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path_data: String,
    #[serde(default = "default_port")]
    pub port: u16
}

fn default_port() -> u16 {
    6767
}

pub fn parse_yaml_file<T: de::DeserializeOwned>(path: &str) -> Result<T, serde_yaml::Error> {
    let path = Path::new(path);
    // Open the path in read-only mode
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open file: {}", why.to_string()),
        Ok(file) => file,
    };
    println!("Parsing file successfully");
    let result: T = serde_yaml::from_reader(&file).unwrap();
    Ok(result)
}