use std::io;
use std::io::Read;
use std::fs::{File, OpenOptions};
use std::path::Path;
use lz4::{EncoderBuilder, Decoder};
use super::provider::FsProvider;

pub struct LZ4Provider;

impl FsProvider for LZ4Provider {
    fn compress(&self, source: &str, dest: &Path) -> io::Result<()> {
        let output_file = OpenOptions::new().create(true).write(true).open(dest)?;
        let mut encoder = EncoderBuilder::new().level(4).build(output_file)?;
        io::copy(&mut source.as_bytes(), &mut encoder)?;
        let (_output, result) = encoder.finish();
        result
    }

    fn decompress(&self, source: &Path, dest: &mut String) -> io::Result<()> {
        let input_file = File::open(source)?;
        let mut decoder = Decoder::new(input_file)?;
        decoder.read_to_string(dest)?;
        return Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_test() {
        let p = LZ4Provider{};
        p.compress(&String::from("hello world"), Path::new("/tmp/test.lz4")).unwrap();
        let mut content = String::from("");
        p.decompress(Path::new("/tmp/test.lz4"), &mut content).unwrap();
        assert_eq!(content, "hello world");
    }
}