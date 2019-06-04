use super::cache;
use super::games_index;
use super::model;
use htmlescape::decode_html;

use chrono::prelude::*;
use regex::Regex;
use select::document::Document;
use select::node::Data;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn get_game_desc(info: &model::GameInfo) -> model::GameInfoFull {
  let mut result = model::GameInfoFull {
    Id: info.Id.clone(),
    Name: info.Name.clone(),
    Genre: info.Genre.clone(),
    Year: info.Year.clone(),
    Platform: info.Platform.clone(),
    Publisher: info.Publisher.clone(),
    Stars: info.Stars,

    AltName: None,
    CoverImageUrl: None,
    Desc: String::from(""),
    UsersStars: 5.0,
    UsersStarsSum: 4,
    Favorites: 0,
    Completed: 0,
    Bookmarks: 0,
    Recomended: Vec::new(),
    Mods: Vec::new(),
    GameGroups: Vec::new(),
    Comments: Vec::new(),
    CommentsThreadId: None,
    Screenshots: Vec::new(),
  };

  //заполняем url скриншотов
  {
    let doc = cache::CacheStruct::load_game_screenshots(info.Id);
    let node = doc.find(
      Name("div").and(Class("main-content")).descendant(
        Name("div").and(Attr("id", "game_screens")).descendant(
          Name("div").and(Attr("id", "screensarea")).descendant(
            Name("ul")
              .and(Class("gamescreens"))
              .descendant(Name("li").and(Class("game_screen"))),
          ),
        ),
      ),
    );

    for n in node {
      let screenshots: Vec<_> = n
        .find(
          Name("div").descendant(
            Name("div")
              .and(Class("screen_img_container"))
              .descendant(Name("a")),
          ),
        )
        .collect();
      if screenshots.len() > 0 {
        let screenshot = screenshots.last().unwrap();
        let thumb = n.find(Name("img")).next().unwrap();
        result.Screenshots.push(model::TScreenshot {
          ThumbUrl: thumb.attr("src").unwrap().to_string(),
          Url: screenshot.attr("href").unwrap().to_string(),
        });
      }
    }
  }

  {
    let doc = cache::CacheStruct::load_game_desc(info.Id);
    {
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("div").and(Attr("id", "reviewtext"))),
        )
        .next()
        .unwrap();
      result.Desc = trim_node_html(&node);
    }
    {
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("span").and(Class("game_alt_names"))),
        )
        .next();
      match node {
        Some(n) => result.AltName = Some(n.text()),
        None => {}
      }
    }
    {
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("table").and(Class("gameinfo")))
            .descendant(Name("td").and(Class("game-cover")))
            .descendant(Name("div").and(Class("game_info_cover")))
            .descendant(Name("img")),
        )
        .next()
        .unwrap();
      let cover = node.attr("src").unwrap().to_string();
      if cover.contains("nocover-rus.gif")
      {
        result.CoverImageUrl = Some(cover);
      }
    }
    {
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("table").and(Class("gameinfo")))
            .descendant(Name("span").and(Attr("itemprop", "aggregateRating")))
            .descendant(Name("img")),
        )
        .next()
        .unwrap();

      parse_game_users_stars(node.attr("title").unwrap(), &mut result);
    }
    {
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("div").and(Attr("id", "game_review")))
            .descendant(Name("div").and(Attr("id", "reviewarea")))
            .descendant(Name("ul").and(Class("game-groups"))),
        )
        .next();
      if node != None {
        let node = node.unwrap();
        for item in node.find(Name("li")) {
          let mut group = model::TGameGroup {
            Name: item.children().next().unwrap().text(),
            Values: Vec::new(),
          };

          for value in item.find(Name("a")) {
            let group_value = model::TGameGroupValue {
              Href: value.attr("href").unwrap().to_string(),
              Title: value.attr("title").unwrap().trim().to_string(),
              Value: value.text().trim().to_string(),
            };
            group.Values.push(group_value);
          }

          result.GameGroups.push(group);
        }
      }
    }
    {
      let nodes: Vec<_> = doc
        .find(
          Name("div")
            .and(Class("bookmark-icon-block"))
            .descendant(Name("ul").and(Class("bookmark-icon-list")))
            .descendant(Name("li")),
        )
        .collect();
      result.Favorites = nodes[0]
        .find(Name("div").and(Class("bookmark-icon-count")))
        .next()
        .unwrap()
        .text()
        .parse()
        .unwrap();
      result.Completed = nodes[1]
        .find(Name("div").and(Class("bookmark-icon-count")))
        .next()
        .unwrap()
        .text()
        .parse()
        .unwrap();
      result.Bookmarks = nodes[2]
        .find(Name("div").and(Class("bookmark-icon-count")))
        .next()
        .unwrap()
        .text()
        .parse()
        .unwrap();
    }
    {
      let nodes = doc.find(
        Name("div")
          .and(Class("main-content"))
          .descendant(Name("table").and(Class("game_content_table")))
          .descendant(Name("tr")),
      );
      for n in nodes {
        let temp = n
          .find(Name("td").descendant(Name("div").and(Class("game_description_col_text"))))
          .next();
        if temp != None {
          let temp = temp.unwrap();
          if temp.text().contains("Рекомендуемые") {
            let recomend_nodes = n.find(
              Name("td")
                .descendant(Name("div").and(Class("middlesmall")))
                .descendant(Name("a").and(Class("game_recommended"))),
            );
            for r in recomend_nodes {
              result.Recomended.push(model::TGameRecomended {
                Id: games_index::get_game_id_from_url(r.attr("href").unwrap()),
                Name: r.text(),
              });
            }
          } else if temp.text().contains("Модификации") {
            let mod_nodes = n.find(
              Name("td")
                .descendant(Name("div").and(Class("middlesmall")))
                .descendant(Name("a").and(Class("game_recommended"))),
            );
            for r in mod_nodes {
              result.Mods.push(model::TGameMod {
                Href: r.attr("href").unwrap().to_string(),
                Name: r.text(),
              });
            }
          }
        }
      }
    }

    {
      //количество страниц комментариев
      let node = doc
        .find(
          Name("div")
            .and(Class("main-content"))
            .descendant(Name("div").and(Attr("id", "comments")))
            .descendant(Name("div").and(Class("game_comments_pager"))),
        )
        .next();
      //если несколько страниц комментариев
      if node != None {
        let pages_count_text = node
          .unwrap()
          .find(Name("span"))
          .next()
          .unwrap()
          .text()
          .to_string();
        lazy_static! {
          static ref RE: Regex = Regex::new(r"Стр\. (\d+)/(\d+)").unwrap();
        }

        let capture = RE.captures_iter(&pages_count_text).next().unwrap();
        let pages_count: i32 = capture[2].parse().unwrap();

        parse_comments(&doc, &mut result.Comments);

        let thread_id = find_thread_id(doc);
        result.CommentsThreadId = Some(thread_id.clone());

        for i in 2..pages_count+1 {
          let subdoc = cache::CacheStruct::load_comments(result.Id, i, &thread_id);
          parse_comments(&subdoc, &mut result.Comments);
        }
      }
      //если комментарии без страниц
      else {
        let node = doc
          .find(
            Name("div")
              .and(Class("main-content"))
              .descendant(Name("div").and(Attr("id", "comments")))
              .descendant(Name("table").and(Class("game_comments_table"))),
          )
          .next();

        if node != None {
          parse_comments(&doc, &mut result.Comments);
        }
      }
    }
  }

  result
}

pub fn parse_comments(doc: &Document, comments: &mut Vec<model::TGameComment>) {
  let nodes = doc.find(
    Name("div").and(
      Class("game_comments_container"))
        .descendant(Name("table").and(Class("game_comments_table")).descendant(Name("tr"))),
  );
  for n in nodes {
    match n.find(Name("td").and(Class("game_comments_row"))).next() {
      Some(node) => {
        let mut new_comment = model::TGameComment {
          UserRef: None,
          UserName: String::from(""),
          Date: chrono::Utc::now(),
          Stars: 0,
          Comment: String::from(""),
        };
        {
          let user_node = node
            .find(
              Name("div")
                .and(Class("middlesmall"))
                .descendant(Name("b"))
                .descendant(Name("a")),
            )
            .next();

          if user_node == None {
            let user_node = node
              .find(Name("div").and(Class("middlesmall")).descendant(Name("b")))
              .next();
            //          new_comment.UserRef = "";
            new_comment.UserName = user_node.unwrap().text();
          } else {
            new_comment.UserRef = Some(user_node.unwrap().attr("href").unwrap().to_string());
            new_comment.UserName = user_node.unwrap().text();
          }
        }
        {
          let comment_node = n
            .find(
              Name("td")
                .and(Class("game_comments_text"))
                .descendant(Name("div").and(Class("middlesmall"))),
            )
            .next()
            .unwrap();
          new_comment.Comment = trim_node_html(&comment_node);
        }
        {
          let node = n
            .find(Name("div").and(Class("middlesmall")).and(Class("red")))
            .next()
            .unwrap();
          new_comment.Date = Utc
            .datetime_from_str(node.text().trim(), "%d.%m.%Y %H:%M")
            .unwrap();
        }
        {
          let node = n.find(Name("img")).next().unwrap();
          new_comment.Stars = parse_comment_user_stars(node.attr("title").unwrap());
        }
        comments.push(new_comment);
      }
      _ => {}
    }
  }
}

pub fn parse_game_users_stars(value: &str, info: &mut model::GameInfoFull) {
  lazy_static! {
    static ref RE: Regex = Regex::new(
      r"Оценка пользователей - ([\d\.]+) из 10. Всего голосов: (\d+)"
    )
    .unwrap();
  }
  let capture = RE.captures_iter(value).next().unwrap();
  info.UsersStars = capture[1].parse().unwrap();
  info.UsersStarsSum = capture[2].parse().unwrap();
}

pub fn parse_comment_user_stars(value: &str) -> i32 {
  if value == "Оценка отсутствует" {
    return -1;
  } else {
    lazy_static! {
      static ref RE: Regex =
        Regex::new(r"Оценка пользователя - (\d+) из 10").unwrap();
    }
    let capture = RE.captures_iter(value).next().unwrap();
    capture[1].parse().unwrap()
  }
}

pub fn find_thread_id(doc: Document) -> String {
  let script_node = doc
    .find(
      Name("div")
        .and(Class("main-content"))
        .descendant(Name("script").and(Attr("type", "text/javascript"))),
    )
    .next()
    .unwrap();

  lazy_static! {
    static ref RE: Regex = Regex::new("\"gamethreadid\":\\s*\"(\\d+)\"").unwrap();
  }
  let text = script_node.text();
  let capture = RE.captures_iter(&text).next().unwrap();
  String::from(&capture[1])
}

pub fn trim_html(input: &str) -> String {
  lazy_static! {
    static ref RE0: Regex = Regex::new(r"[ ]{2,}").unwrap();
    static ref RE1: Regex = Regex::new(r"[\r\n\t]").unwrap();
  }
  //2 и более пробелов заменяем одним
  let s0 = RE0.replace_all(input, " ");
  //удаляем все переносы и табуляцию
  let s1 = RE1.replace_all(&s0, "");
  String::from(s1.trim())
}

pub fn convert_tags(node: &Node, write_tags: bool) -> String {
  let mut result: Vec<String> = Vec::new();

  match node.data() {
    Data::Text(ref text) => {
      result.push(String::from(text));
    }
    Data::Element(ref name, values) => {
      //let has_child = node.children().count() > 0;
      if write_tags {
        result.push(format!("[{}]", name.local));
      }

      for child in node.children() {
        result.push(convert_tags(&child, true));
      }
      if write_tags && &name.local != "br" {
        result.push(format!("[/{}]", name.local));
      }
    }
    Data::Comment(ref comment) => {
      println!("comment={}", comment);
    }
  }
  result.join("")
}

pub fn trim_node_html(input: &Node) -> String {
  trim_html(&convert_tags(input, false))
}

pub fn enrich_games_list(info: &Vec<model::GameInfo>) -> Vec<model::GameInfoFull> {
  let mut result: Vec<model::GameInfoFull> = Vec::new();
  for item in info {
    result.push(get_game_desc(item));
  }

  result
}
