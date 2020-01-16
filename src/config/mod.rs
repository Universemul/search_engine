use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path_data: String,
    #[serde(default = "default_port")]
    pub port: i16
}

fn default_port() -> i16 {
    6767
}

pub fn parse() -> Result<Config, serde_yaml::Error> {
    let path = Path::new("config.yaml");
    // Open the path in read-only mode
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open config file: {}", why.to_string()),
        Ok(file) => file,
    };
    println!("Parsing config file successfully");
    // Read yaml config file.
    let config: Config = serde_yaml::from_reader(&file)?;
    Ok(config)
}