pub mod routes;
pub mod models;

use crate::internal;
use crate::fs::provider::FsProvider;
use crate::fs::lz4_provider::LZ4Provider;
use std::collections::HashMap;
use std::path::Path;

#[allow(dead_code)]
fn write_metafile(index: &str, segment_count: i16, path_data: String) {
    let p = LZ4Provider{};
    let m: HashMap<String, String> = HashMap::new();
    let index = internal::models::Index {name: index.to_string(), segments_count: segment_count, mapping: m, size: 0.0};
    let path = format!("{}/{}/.meta", path_data, index);
    p.compress(&serde_yaml::to_string(&index).unwrap(), Path::new(&path)).unwrap();
}