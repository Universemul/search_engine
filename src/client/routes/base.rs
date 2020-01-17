use std::fs;
use std::path::Path;

use crate::client::models::Index;
use crate::fs::lz4_provider::LZ4Provider;
use crate::fs::provider::FsProvider;

pub fn home(_request: &mut nickel::Request) ->  String {
    // Change when the cache is done
    let path = "/tmp/searchengine/";
    let paths = fs::read_dir(path).unwrap();
    let provider = LZ4Provider{};
    let mut result: Vec<Index> = Vec::with_capacity(10);

    for p in paths {
        let entry = p.unwrap();
        if entry.path().is_dir() {
            let mut _dir = fs::read_dir(entry.path()).unwrap();
            let meta_file = _dir.find(|x| x.as_ref().unwrap().path().file_name().unwrap() == ".meta").unwrap();
            let mut content = String::from("");
            provider.decompress(Path::new(&meta_file.unwrap().path()), &mut content).unwrap();
            let data_index: Index = serde_yaml::from_str(&content).unwrap();
            result.push(data_index);
        }
    }
    format!("{:?}", result).to_owned()
}