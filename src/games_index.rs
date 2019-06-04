use super::cache;
use super::model;
use super::model::GameInfo;
use htmlescape::decode_html;

use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

pub fn parse_games_catalog() -> Vec<model::GameInfo> {
    let page = cache::CacheStruct::load_page(1);
    let pages_count = find_max_pages(page);

    let mut result: Vec<GameInfo> = Vec::new();

    for i in 1..pages_count+1 {
        result.extend(parse_page(cache::CacheStruct::load_page(i)));
    }
    
    result.sort_by(|a, b| a.Id.cmp(&b.Id));

    check_duplicates(&result);

    result
}
pub fn check_duplicates(array: &Vec<model::GameInfo>)
{
    let mut last_val = array[0].Id;
    for i in 1..array.len()
    {
        if last_val==array[i].Id
        {
            panic!("Дубликат Id в списке GameInfo");
        }
        last_val = array[i].Id;
    }
}
pub fn parse_page(doc: Document) -> Vec<model::GameInfo> {
    let mut result: Vec<GameInfo> = Vec::new();

    let games_table = doc.find(
        Name("div")
            .and(Class("main-content"))
            .descendant(Name("table"))
            .descendant(Name("tr")),
    );
    for tr in games_table {
        let game_nodes: Vec<_> = tr.children().filter(|x| x.name() == Some("td")).collect();
        if game_nodes.len() >= 6 {
            let title_node = game_nodes[0].find(Name("a")).next().unwrap();
            let stars = game_nodes[5]
                .find(Name("img"))
                .next()
                .unwrap()
                .attr("title")
                .unwrap();

            let game_info = GameInfo {
                Name: title_node.attr("title").unwrap().to_string(),
                Id: get_game_id_from_url(title_node.attr("href").unwrap()),
                Genre: game_nodes[1].find(Name("a")).next().unwrap().text(),
                Year: game_nodes[2].find(Name("a")).next().unwrap().text(),
                Platform: game_nodes[3].find(Name("a")).next().unwrap().text(),
                Publisher: game_nodes[4].find(Name("a")).next().unwrap().text(),

                Stars: {
                    lazy_static! {
                        static ref RE: Regex =
                            Regex::new(r"Оценка рецензента - (\d+) из 10")
                                .unwrap();
                    }
                    match stars {
                        "Оценка отсутствует" => -1,
                        _ => {
                            let capture = RE.captures_iter(stars).next().unwrap();
                            let result = capture[1].parse().unwrap();
                            result
                        }
                    }
                },
            };
            result.push(game_info);
        }
    }
    result
}

pub fn get_game_id_from_url(url: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/game/(\d+)\.html").unwrap();
    }
    let capture = RE.captures_iter(url).next().unwrap();
    let result = capture[1].parse().unwrap();
    result
}

fn find_max_pages(doc: Document) -> i32 {
    let pages_el = doc.find(Name("ul").and(Class("pager"))).last().unwrap();
    let page = pages_el.last_child().unwrap();
    let pages_count;
    match page.text().parse::<i32>() {
        Ok(n) => pages_count = n,
        Err(_e) => {
            let prev_child = page.prev().unwrap();
            match prev_child.text().parse::<i32>() {
                Ok(n) => pages_count = n,
                Err(_e) => {
                    panic!("Отсутствует кол-во страниц");
                }
            }
        }
    }

    return pages_count;
}
