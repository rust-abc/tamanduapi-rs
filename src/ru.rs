use std::collections::{HashMap, HashSet};

use select::{
    document::Document,
    predicate::{Attr, Class, Element, Name, Predicate, Text},
};
use surf;
// use reqwest;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Days {
    Seg,
    Ter,
    Qua,
    Qui,
    Sex,
    Sab,
    Dom,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Lunch(Food),
    Dinner(Food),
    Salad(String),
    Dessert(String),
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Food {
    main_course: String,
    veggie:      String,
    extra:       String,
}

impl Food {
    pub const fn new() -> Self {
        Self { main_course: String::new(), veggie: String::new(), extra: String::new() }
    }

    pub fn from_values(main_course: String, veggie: String, extra: String) -> Self {
        Self { main_course, veggie, extra }
    }
}

pub async fn get_table() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use Days::*;

    let doc = surf::get(
        "http://proap.ufabc.edu.br/nutricao-e-restaurantes-universitarios/cardapio-semanal",
    )
    .set_header("Content-Length", "0")
    .await?
    .body_string()
    .await?;

    let html = Document::from(doc.as_str());
    let card_semanal = html.find(Class("cardapio-semanal")).nth(0).unwrap();
    let html_table = card_semanal.find(Name("table")).nth(0).unwrap();
    dbg!(&html_table);

    let mut table: HashMap<Days, HashSet<Category>> = HashMap::new();

    for tr in html_table.find(Name("tr")) {
        match tr.attr("class") {
            Some("cardapio-hoje") => {
                let day_str = tr.find(Name("td")).nth(0).unwrap().text();
                let day = match day_str {
                    _s if day_str.contains("Seg") => Seg,
                    _s if day_str.contains("Ter") => Ter,
                    _s if day_str.contains("Qua") => Qua,
                    _s if day_str.contains("Qui") => Qui,
                    _s if day_str.contains("Sex") => Sex,
                    _s if day_str.contains("Sab") => Sab,
                    _ => Dom,
                };
                // dbg!(&day);

                for cat in tr.find(Name("td")) {
                    match cat.text().as_str() {
                        "Almoço" => {
                            let main_course = cat.find(Name("li")).nth(0).unwrap().text();
                            let veggie = cat.find(Name("li")).nth(1).unwrap().text();
                            let extra = cat.find(Name("li")).nth(1).unwrap().text();

                            dbg!(main_course, veggie, extra);
                        },
                        "Janta" => {},
                        "Saladas" => {},
                        "Sobremesas" => {},
                        _ => {}
                    }
                }

                // table.insert(day, v: V);
            },
            Some(_) => {},
            None => {},
        }
    }



    // dbg!(html);
    Ok(doc)
}
