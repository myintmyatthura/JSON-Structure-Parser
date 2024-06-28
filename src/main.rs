pub mod constants;
pub mod utils;

fn main() {
    use constants::dictionary::dictionary;
    use utils::json::{deserialize, parse_all, serialize_to_json_pretty};
    use utils::read_text::{read_text, write_text};

    let modified_contents = read_text().replace("null", "\"null\"");
    let my_dict = dictionary();

    if let Some(collections) = deserialize(modified_contents) {
        let modified_collections = parse_all(collections, &my_dict);
        let my_string = serialize_to_json_pretty(&modified_collections);
        write_text(&my_string);
    } else {
        eprintln!("Failed to deserialize the JSON content.");
    }
}
