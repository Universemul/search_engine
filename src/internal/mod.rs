pub mod routes;
pub mod models;

use crate::internal;
use crate::warp::Filter;
use crate::fs::provider::FsProvider;
use crate::fs::lz4_provider::LZ4Provider;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

pub async fn startup(port: u16, path_data: &str) -> () {
    let _p = path_data.to_string();
    let index = warp::path::end().map(move || {
        routes::base::home(&_p)
    });
    let index_route = routes::index_route(path_data);
    let routes = warp::get().and(index.or(index_route));
    println!("{}", format!("Booting Rest API : {}:{}", "127.0.0.1", port));
    warp::serve(routes).run(([127, 0, 0, 1], port)).await
}

#[allow(dead_code)]
pub fn write_metafile(index_name: &str, segment_count: i16, path_data: String) {
    let p = LZ4Provider{};
    let m: HashMap<String, String> = HashMap::new();
    let index = internal::models::Index {name: index_name.to_string(), segments_count: segment_count, mapping: m, size: 0.0, is_ok: true};
    let filepath = format!("{}{}/.meta", path_data, index_name);
    let path = Path::new(&filepath);
    if !Path::new(path).exists() {
        println!("{:?} created", path);
        File::create(path).unwrap();
    }
    p.compress(&serde_yaml::to_string(&index).unwrap(), path).unwrap();
}