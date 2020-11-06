extern crate lz4;
extern crate serde;
extern crate serde_yaml;
extern crate warp;


pub mod internal;
pub mod config;
pub mod fs;

#[tokio::main]
async fn main() {
    // Loading config
    let conf: config::Config = config::parse_yaml_file("config.yaml").unwrap();
    // Load the cache
    
    // Ensure that every index has .metafile
    
    // Starting the api
    internal::startup(conf.port, &conf.path_data).await;
    // Add here logging for good starting
}