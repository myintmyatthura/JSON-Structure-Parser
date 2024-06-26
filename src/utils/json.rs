use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonDict {
    pub name: String,
    pub mode: String,
    pub r#type: String,
    pub description: String,
    pub fields: Vec<String>,
}

impl JsonDict {
    // Method to modify field1 and return the entire struct
    fn parse_description(mut self, data_dict: &HashMap<&str, &str>) -> Self {
        let parts_name: Vec<&str> = self.name.split('_').collect();

        let mut new_parts: Vec<String> = Vec::new();
        for part in &parts_name {
            if let Some(&value) = data_dict.get(part) {
                new_parts.push(value.to_string());
            } else {
                new_parts.push(part.to_string());
            }
        }

        let final_value = new_parts.join(" ");
        self.description = final_value;

        self
    }
}

/*
This is an accurate representation of what our json object should look like
*/
pub fn create_json() -> JsonDict {
    JsonDict {
        name: "MI_SUB".to_string(),
        mode: "NULLABLE".to_string(),
        r#type: "STRING".to_string(),
        description: "\"null\"".to_string(),
        fields: Vec::from([]),
    }
}

pub fn deserialize(input: String) -> Option<Vec<JsonDict>> {
    // Deserialize the JSON string into your struct object
    let deserialized_obj: Result<Vec<JsonDict>, _> = from_str(&input);

    match deserialized_obj {
        Ok(obj) => Some(obj),
        Err(err) => {
            eprintln!("Error deserializing JSON: {:?}", err);
            None
        }
    }
}

pub fn parse_string(input: JsonDict, dict: &HashMap<&str, &str>) -> JsonDict {
    let output = input.parse_description(dict);

    println!("{:#?}", output);
    output
}

pub fn parse_all(input: Vec<JsonDict>, dict: &HashMap<&str, &str>) -> Vec<JsonDict> {
    let iter_dict = input.iter().map(|x| x.clone().parse_description(dict));
    iter_dict.collect()
}

pub fn serialize_to_json_pretty(input: &Vec<JsonDict>) -> String {
    match to_string_pretty(input) {
        Ok(json_str) => json_str,
        Err(err) => {
            eprintln!("Error serializing JSON: {:?}", err);
            String::new()
        }
    }
}
