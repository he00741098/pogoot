use nanoid::nanoid;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
fn main() {
    let mut notes = vec![];
    // let random = rand::thread_rng();
    for i in 0..100000{
        notes.push(Notecard{
            front: nanoid!(),
            back: nanoid!(),
        });
    }
    let to_serialize = serde_json::to_string(&notes).unwrap();
    println!("To_serialize len: {}", to_serialize.as_bytes().to_vec().len());
    let flate2_vec = to_serialize.clone();
    let zstd_vec = to_serialize.clone();
    let now = chrono::Utc::now();
    let mut e = ZlibEncoder::new(Vec::with_capacity(6500000), Compression::default());
    e.write_all(flate2_vec.as_bytes());
    let compressed_bytes = e.finish();
    let duration = chrono::Utc::now().signed_duration_since(now);
    let compressed_bytes = compressed_bytes.unwrap();
    println!("Flate2 compressed len: {}, Duration: {:?}", compressed_bytes.len(), duration);

    let now = chrono::Utc::now();
    let compressed = zstd::stream::encode_all(zstd_vec.as_bytes(), 0).unwrap();
    let duration = chrono::Utc::now().signed_duration_since(now);
    println!("Zstd compressed len: {}, Duration: {:?}", compressed.len(), duration);

    //test decompression
    let now = chrono::Utc::now();
    let decompressed = zstd::stream::decode_all(&*compressed).unwrap();
    let duration = chrono::Utc::now().signed_duration_since(now);
    println!("Zstd decompressed len: {}, Duration: {:?}", decompressed.len(), duration);

    let now = chrono::Utc::now();
    let mut d = flate2::write::ZlibDecoder::new(Vec::new());
    d.write_all(&compressed_bytes);
    let decompressed_bytes = d.finish();
    let duration = chrono::Utc::now().signed_duration_since(now);
    println!("Flate2 decompressed len: {}, Duration: {:?}", decompressed_bytes.unwrap().len(), duration);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notecard{
    pub front: String,
    pub back: String,
}

