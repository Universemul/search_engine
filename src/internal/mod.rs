pub mod routes;
pub mod models;

use nickel::{Nickel, HttpRouter};

use crate::internal;
use crate::fs::provider::FsProvider;
use crate::fs::lz4_provider::LZ4Provider;
use std::collections::HashMap;
use std::path::Path;

pub fn startup(port: i16, path_data: &str) -> () {
    let mut server = Nickel::new();
    let p = path_data.to_string();
    server.get("/", middleware! {  |request, mut response|
        routes::base::home(request, &p)
    });
    routes::index_route(&mut server, path_data);

    let addr = format!("127.0.0.1:{}", port);
    println!("Start Rest Api successfully");
    server.listen(addr).unwrap();
}

#[allow(dead_code)]
fn write_metafile(index: &str, segment_count: i16, path_data: String) {
    let p = LZ4Provider{};
    let m: HashMap<String, String> = HashMap::new();
    let index = internal::models::Index {name: index.to_string(), segments_count: segment_count, mapping: m, size: 0.0};
    let path = format!("{}/{}/.meta", path_data, index);
    p.compress(&serde_yaml::to_string(&index).unwrap(), Path::new(&path)).unwrap();
}