pub mod constants;
pub mod utils;

fn main() {
    let parsed_string = utils::read_text::read_text();
    let collections = utils::json::ser(parsed_string).unwrap();
    println!("{:#?}", collections);
    // implement string-matching algorithm..
}
