use lz4::{EncoderBuilder, Decoder};
use std::io::{Read, Write};

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = EncoderBuilder::new().level(4).build(Vec::new()).unwrap();
    encoder.write_all(data).expect("Failed to write");
    let (compressed, result) = encoder.finish();
    result.expect("Failed to compress");
    compressed
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = Decoder::new(data).expect("Failed to create decoder");
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("Failed to decompress");
    decompressed
}
