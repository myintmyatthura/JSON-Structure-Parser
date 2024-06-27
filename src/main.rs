pub mod constants;
pub mod utils;

fn main() {
    let parsed_string = utils::read_text::read_text();
    let my_dict = constants::dictionary::dictionary();
    let modified_contents = parsed_string.replace("null", "\"null\"");

    let collections = utils::json::serialize(modified_contents).unwrap();

    let modified_collections = utils::json::parse_all(collections, &my_dict);

    println!("{:#?}", modified_collections);

    // let new_json_object = utils::json::create_json();
    // let my_dict = constants::dictionary::dictionary();
    // let _my_output = utils::json::parse_string(new_json_object, &my_dict);

    // implement string-matching algorithm..
}
