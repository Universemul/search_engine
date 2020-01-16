use std::path::{Path};
use std::io;

pub trait FsProvider {
    fn compress(&self, source: &str, dest: &Path) -> io::Result<()>; // Define  err
    fn decompress(&self, source: &Path, dest: &mut String) -> io::Result<()>; // Define err
}