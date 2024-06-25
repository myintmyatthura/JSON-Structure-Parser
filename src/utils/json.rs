use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonDict {
    pub name: String,
    pub mode: String,
    pub r#type: String,
    pub description: Option<String>,
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
        self.description = Some(final_value);

        self
    }
}

pub fn create_json() -> JsonDict {
    return JsonDict {
        name: "SS_MM".to_string(),
        mode: "NULLABLE".to_string(),
        r#type: "STRING".to_string(),
        description: None,
        fields: Vec::from([]),
    };
}

pub fn ser(input: String) -> Option<Vec<JsonDict>> {
    //Deserialize the JSON string into your struct object
    let deserialized_obj: Result<Vec<JsonDict>, _> = from_str(&input);

    match deserialized_obj {
        Ok(obj) => {
            return Some(obj);
        }
        Err(err) => {
            eprintln!("Error deserializing JSON: {:?}", err);
            return None;
        }
    }
}

pub fn parse_string(input: JsonDict, dict: &HashMap<&str, &str>) -> JsonDict {
    let output = input.parse_description(dict);

    return output;
}

pub fn deser() {
    println!("Need implementation!");
}

pub fn run_json() {
    println!("Need implementation!");
}
