use tamanduapi::ru;
use futures::executor::block_on;
// use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(html) = block_on(ru::get_table()) {
        // println!("{}", html);
    };

    Ok(())
}
