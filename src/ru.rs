use surf;
use select::{document::Document, predicate::{Predicate, Attr, Class, Name}};
// use reqwest;

pub async fn get_table() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let doc = surf::get(
        "http://proap.ufabc.edu.br/nutricao-e-restaurantes-universitarios/cardapio-semanal",
    )
    .set_header("Content-Length", "0")
    .await?
    .body_string()
    .await?;

    let html = Document::from(doc.as_str());
    let card_semanal = html.find(Class("cardapio-semanal")).nth(0).unwrap();
    let table = card_semanal.find(Name("table")).nth(0).unwrap();

    for tr in table.find(Name("tr")) {

    }



    // dbg!(html);
    Ok(doc)
}
