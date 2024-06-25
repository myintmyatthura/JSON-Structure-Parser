pub mod constants;
pub mod utils;

fn main() {
    let parsed_string = utils::read_text::read_text();
    let collections = utils::json::ser(parsed_string).unwrap();
    println!("{:#?}", collections);

    // let collection_iter = collections.iter();
    // let new_json_object = utils::json::create_json();
    // let my_dict = constants::dictionary::dictionary();
    // let _my_output = utils::json::parse_string(new_json_object, &my_dict);

    // implement string-matching algorithm..
}
