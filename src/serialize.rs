use serde_json;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use std::io::prelude::*;
use zip::write::FileOptions;

use super::model;
use chrono::prelude::*;

const FULL_GAMES_LIST_ARCHIVE_ENTRY: &str = "games.json";
const FULL_GAMES_LIST_ZIP: &str = "data/games.json.zip";

const CATALOG_ARCHIVE_ENTRY: &str = "catalog.json";
const CATALOG_ZIP: &str = "data/catalog.json.zip";

pub fn serialize<T>(value: &T, stream: &mut std::fs::File)
where
    T: serde::Serialize,
{
    let j = serde_json::to_string_pretty(&value).unwrap();
    stream.write(j.as_bytes()).unwrap();
}

pub fn serialize_game_info(value: &Vec<model::GameInfoFull>) {
    let file = std::fs::File::create(&FULL_GAMES_LIST_ZIP).unwrap();
    let buffer = std::io::BufWriter::with_capacity(65536, file);
    let mut zip = zip::ZipWriter::new(buffer);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
    zip.start_file(CATALOG_ARCHIVE_ENTRY, options).unwrap();
    serde_json::to_writer_pretty(&mut zip, &value).unwrap();
    zip.finish().unwrap();
}

pub fn serialize_catalog(value: &Vec<model::GameInfo>) {
    let file = std::fs::File::create(&CATALOG_ZIP).unwrap();
    let buffer = std::io::BufWriter::with_capacity(65536, file);
    let mut zip = zip::ZipWriter::new(buffer);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
    zip.start_file(CATALOG_ARCHIVE_ENTRY, options).unwrap();
    serde_json::to_writer_pretty(&mut zip, &value).unwrap();
    zip.finish().unwrap();
}
