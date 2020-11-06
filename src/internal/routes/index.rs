use std::fs;
use std::path::Path;

use crate::internal::models::Index;
use crate::fs::lz4_provider::LZ4Provider;
use crate::fs::provider::FsProvider;

pub fn root(index_name: String, path_data: &str) -> String {
    let path = format!("{}/{}", path_data, index_name);
    let mut _dir = fs::read_dir(path).unwrap();
    let provider = LZ4Provider{};
    let meta_file = _dir.find(|x| x.as_ref().unwrap().path().file_name().unwrap() == ".meta").unwrap();
    let m_path = meta_file.unwrap().path();
    let file_size = fs::metadata(m_path.clone()).unwrap().len();
    let mut content = String::from("");
    provider.decompress(Path::new(&m_path), &mut content).unwrap();
    let mut data_index: Index = serde_yaml::from_str(&content).unwrap();
    data_index.size = file_size as f64;
    "INDEX IS OK".to_string()
}

pub fn count(_index_name: String) -> String {
    //TODO : Count the number of document inside an index
    format!("COUNT {}", "OK")
}

pub fn search(_index_name: String) -> String {
    //TODO : Search document in index
    format!("SEARCH {}", "OK")
}