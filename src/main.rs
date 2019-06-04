extern crate reqwest;
extern crate select;
extern crate serde_json;
extern crate serde;
extern crate chrono;
extern crate htmlescape;
extern crate html_entities;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate encoding_rs;
extern crate encoding_rs_io;
extern crate zip;
extern crate walkdir;

mod cache;
mod model;
mod games_index;
mod game_desc;
mod serialize;
mod zipdir;

use std::io;

use chrono::prelude::*;

use std::fs::File;

fn main() -> Result<(), Box<std::error::Error>> {

    let start_time = Local::now();
    println!("Parser started {}", start_time.format("%Y.%m.%d %H-%M-%S"));

    //zipdir::doit("data/pages_html", "pages_html.zip").unwrap();

    //println!("Zip finished {} sec", (Local::now()-start_time).num_seconds());

    // let mut _line = String::new();
    // io::stdin().read_line(&mut _line)
    //     .expect("Failed to read line");

    let info = games_index::parse_games_catalog(); 

    let full_info = game_desc::enrich_games_list(&info);

    println!("Pages list parsed {}", Local::now().format("%Y.%m.%d %H-%M-%S"));

    serialize::serialize_game_info(&full_info);

    println!("Zip created {}", Local::now().format("%Y.%m.%d %H-%M-%S"));
    
    println!("Screenshots downloaded {}", Local::now().format("%Y.%m.%d %H-%M-%S"));

    println!("All time {} sec", (Local::now()-start_time).num_seconds());

    println!("Finished");

    let mut _line = String::new();
    io::stdin().read_line(&mut _line)
        .expect("Failed to read line");

    Ok(())
}

