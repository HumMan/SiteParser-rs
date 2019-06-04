extern crate reqwest;

use select::document::Document;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

const TARGET_SITE: &str = "https://www.old-games.ru/";

#[derive(Debug)]
pub struct CacheStruct {
    pub list: HashMap<String, String>,
}

impl CacheStruct {
    pub fn download(url: &String) -> reqwest::Response {
        reqwest::get(url).expect(&format!("request failed - {}", url))
    }

    pub fn load_page(id: i32) -> Document {
        let path = format!("data/pages_html/{}.html.cached", id);
        if Path::new(&path).exists() {
            let file = File::open(path).unwrap();
            let doc = Document::from_read(file).unwrap();
            doc
        } else {
            let mut response =
                CacheStruct::download(&format!("{}catalog/?page={}", TARGET_SITE, id));
            let mut f = File::create(path).unwrap();
            response.copy_to(&mut f).unwrap();
            let doc = Document::from_read(response).unwrap();
            doc
        }
    }
    pub fn load_comments(game_id: i32, id: i32, thread_id: &str) -> Document {
        let path = format!("data/pages_html/{}/comments/{}.html.cached", game_id, id);
        if Path::new(&path).exists() {
            let file = File::open(path).unwrap();
            let transcoded = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(file);
            let doc = Document::from_read(transcoded).unwrap();
            doc
        } else {
            let client = reqwest::Client::new();
            let url = format!("{}game/game_comments.php", TARGET_SITE);

            let mut params = HashMap::new();
            params.insert("gameid", game_id.to_string());
            params.insert("gamethreadid", String::from(thread_id));
            params.insert("page", id.to_string());

            let mut response = client
                .post(&url)
                .form(&params)
                .header( reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded; charset=UTF-8")
                .send().unwrap();

            let mut f = File::create(path).unwrap();
            response.copy_to(&mut f).unwrap();
            let doc = Document::from_read(response).unwrap();
            doc
        }
    }
    pub fn load_game_desc(id: i32) -> Document {
        let path = format!("data/pages_html/{}/desc.html.cached", id);
        if Path::new(&path).exists() {
            let file = File::open(path).unwrap();
            let doc = Document::from_read(file).unwrap();
            doc
        } else {
            let mut response =
                CacheStruct::download(&format!("{}game/{}.html", TARGET_SITE, id));
            let mut f = File::create(path).unwrap();
            response.copy_to(&mut f).unwrap();
            let doc = Document::from_read(response).unwrap();
            doc
        }
    }
    pub fn load_game_screenshots(id: i32) -> Document {
        let path = format!("data/pages_html/{}/screenshots.html.cached", id);
        if Path::new(&path).exists() {
            let file = File::open(path).unwrap();
            let doc = Document::from_read(file).unwrap();
            doc
        } else {
            let mut response =
                CacheStruct::download(&format!("{}game/screenshots/{}.html", TARGET_SITE, id));
            let mut f = File::create(path).unwrap();
            response.copy_to(&mut f).unwrap();
            let doc = Document::from_read(response).unwrap();
            doc
        }
    }
}
