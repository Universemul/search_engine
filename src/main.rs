extern crate lz4;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate nickel;

pub mod internal;
pub mod config;
pub mod fs;

fn main() {
    // Loading config
    let conf: config::Config = config::parse_yaml_file("config.yaml").unwrap();
    // Load the cache
    
    // Starting the api
    internal::startup(conf.port, &conf.path_data);
}