
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TScreenshot {
    pub ThumbUrl: String,
    pub Url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TGameGroupValue {
    pub Href: String,
    pub Title: String,
    pub Value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TGameGroup {
    pub Name: String,
    pub Values : Vec<TGameGroupValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TGameComment {
    pub UserRef: Option<String>,
    pub UserName: String,
    pub Date: DateTime<Utc>,
    pub Stars: i32,
    pub Comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TGameRecomended {
    pub Name: String,
    pub Id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct TGameMod {
    pub Name: String,
    pub Href: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct GameInfo {
    pub Id: i32,
    pub Name: String,
    pub Genre: String,
    pub Year: String,
    pub Platform: String,
    pub Publisher: String,
    pub Stars: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] 
pub struct GameInfoFull {
    pub Id: i32,
    pub Name: String,   
    pub AltName: Option<String>,
    pub CoverImageUrl: Option<String>,
    pub Genre: String,
    pub Year: String,
    pub Platform: String,
    pub Publisher: String,
    pub Stars: i32,

    

    pub Desc: String,

    pub UsersStars: f32,
    pub UsersStarsSum: i32,

    pub Favorites: i32,
    pub Completed: i32,
    pub Bookmarks: i32,

    pub Recomended: Vec<TGameRecomended>,
    pub Mods: Vec<TGameMod>,
    pub GameGroups: Vec<TGameGroup>,
    pub Comments: Vec<TGameComment>,
    pub CommentsThreadId: Option<String>,
    pub Screenshots: Vec<TScreenshot>,
}