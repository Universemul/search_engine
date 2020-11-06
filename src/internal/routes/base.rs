use std::fs;
use std::path::Path;

use crate::internal::models::Index;
use crate::fs::lz4_provider::LZ4Provider;
use crate::fs::provider::FsProvider;

pub fn home(path: & str) -> String {
    let paths = fs::read_dir(path).unwrap();
    let provider = LZ4Provider{};
    let mut indexes: Vec<Index> = Vec::with_capacity(10);

    for p in paths {
        let entry = p.unwrap();
        if entry.path().is_dir() {
            let mut _dir = fs::read_dir(entry.path()).unwrap();
            let meta_file = _dir.find(|x| x.as_ref().unwrap().path().file_name().unwrap() == ".meta").unwrap();
            let m_path = meta_file.unwrap().path();
            let file_size = fs::metadata(m_path.clone()).unwrap().len();
            let mut content = String::from("");
            provider.decompress(Path::new(&m_path), &mut content).unwrap();
            let mut data_index: Index = serde_yaml::from_str(&content).unwrap();
            data_index.size = file_size as f64;
            indexes.push(data_index);
        }
    }
    let mut s = String::new();

    for index in indexes {
        s = format!("{}{} {} {}\n", s, index.name, index.segments_count, index.size);
    }
    s
}