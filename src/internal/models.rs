use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    #[serde(default = "default_size")]
    pub size: f64,
    pub segments_count: i16,
    pub mapping: HashMap<String, String>
}

fn default_size() -> f64 {
    0.0
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