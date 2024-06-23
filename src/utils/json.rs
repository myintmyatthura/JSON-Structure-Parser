

use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonDict {
    pub name: String,
    pub mode: String,
    pub r#type: String,
    pub description: Option<String>,
    pub fields: Vec<String>,
}

pub fn ser(input: String) -> Option<Vec<JsonDict>> {
    //Deserialize the JSON string into your struct object
    let deserialized_obj: Result<Vec<JsonDict>, _> = from_str(&input);

    match deserialized_obj {
        Ok(obj) => {
            return Some(obj);
            // Modify the struct fields as needed
            // For example:
            // obj.field1 = "New Value".to_string();

            // Serialize the modified object back to JSON
            // let serialized_json = to_string_pretty(&obj).unwrap();
            // println!("Serialized JSON: {}", serialized_json);
        }
        Err(err) => {
            eprintln!("Error deserializing JSON: {:?}", err);
            return None;
        }
    }
}

pub fn deser() {
    println!("Need implementation!");
}

pub fn run_json() {
    println!("Need implementation!");
}
