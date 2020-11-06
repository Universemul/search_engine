use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    #[serde(default = "default_size")]
    #[serde(skip_serializing, skip_deserializing)]
    pub size: f64,
    #[serde(default = "default_is_ok")]
    #[serde(skip_serializing, skip_deserializing)]
    pub is_ok: bool,
    pub segments_count: i16,
    pub mapping: HashMap<String, String>
}

fn default_size() -> f64 {
    0.0
}

fn default_is_ok() -> bool {
    true
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  Number of Segments :{}",
            self.name, self.segments_count
        )
    }
}