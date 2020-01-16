extern crate lz4;
extern crate serde;
extern crate serde_yaml;
#[macro_use] extern crate nickel;

pub mod client;
pub mod config;
pub mod fs;

fn main() {
    // Thing about loading config
    let config = config::parse().unwrap(); //check error
    // Starting the api
    client::routes::startup(&config);
}
