pub mod constants;
pub mod utils;

fn main() {
    // let parsed_string = utils::read_text::read_text();
    // let modified_contents = parsed_string.replace("null", "\"null\"");
    // println!("{:#?}", modified_contents);
    // let collections = utils::json::ser(modified_contents).unwrap();
    // println!("{:#?}", collections);

    let new_json_object = utils::json::create_json();
    let my_dict = constants::dictionary::dictionary();
    let _my_output = utils::json::parse_string(new_json_object, &my_dict);

    // implement string-matching algorithm..
}
