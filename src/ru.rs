use std::collections::HashMap;

use surf;
use select::{document::Document, predicate::{Predicate, Attr, Class, Name}};
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

#[derive(Debug, Default, PartialEq, PartialOrd, Hash)]
pub struct Food {
    main_course: String,
    veggie: String,
    extra: String,
    salads: String,
    dessert: String,
    today: bool,
}

impl Food {

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

    let mut table: HashMap<Days, HashMap<&str, Food>> = HashMap::new();
    for tr in html_table.find(Name("tr")) {
        match tr.attr("class") {
            Some("cardapio-hoje") => {
                let day_str = tr.find(Name("td")).nth(0).unwrap().text();
                let day = match day_str {
                    s if day_str.contains("Seg") => Seg,
                    s if day_str.contains("Ter") => Ter,
                    s if day_str.contains("Qua") => Qua,
                    s if day_str.contains("Qui") => Qui,
                    s if day_str.contains("Sex") => Sex,
                    s if day_str.contains("Sab") => Sab,
                    _ => Dom,
                };

                dbg!(tr.prev(), tr.next());

                // table.insert(day, v: V);
            },
            Some(_) => {},
            None => {}
        }
    }



    // dbg!(html);
    Ok(doc)
}
