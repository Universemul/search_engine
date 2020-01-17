extern crate lz4;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate nickel;

pub mod client;
pub mod config;
pub mod fs;

/*
use crate::fs::provider::FsProvider;
use std::path::Path;
use std::collections::HashMap;

// Use to write some data in .meta file
fn write_meta(name: &str, size: f64) {
    let path = format!("/tmp/searchengine/{}/.meta", name);
    let mapping: HashMap<String, String> = HashMap::new();
    let s = client::models::Index {name: name.to_string(), size: size, mapping: mapping};
    let p = fs::lz4_provider::LZ4Provider{};
    p.compress(&serde_yaml::to_string(&s).unwrap(), Path::new(&path)).unwrap();
}*/

fn main() {
    // Loading config
    let conf = config::parse().unwrap(); //check error
    // Loading FS
    
    // Load the cache
    
    // Starting the api
    client::routes::startup(conf.port);
}
