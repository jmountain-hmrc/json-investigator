#[macro_use]
extern crate serde_json;

use std::fs::File;
use serde_json::Value;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error};
use std::env;
use std::collections::HashMap;

// Could have: Intersection of keys, median/mode/mean of intersected keys, keys unique to object

struct BaseData {
    keys: HashMap<String, Vec<Value>>
}

fn macerate_objects(path: &String) -> std::io::Result<BaseData> { 
    let contents = fs::read_to_string(path).expect("Could not read file");
    let parsed_json : Value = serde_json::from_str(&contents).unwrap();

    let mut base_data = BaseData {
        keys: HashMap::new()
    };

    for item in parsed_json.as_array().unwrap() {
        for pair in item.as_object().unwrap() {
            let (name, val) = pair;
            
            match base_data.keys.get(name) {
                Some(values) => {
                    let mut created_vector = values.clone();
                    created_vector.push(val.clone());
                    base_data.keys.insert(name.to_string(), created_vector);
                },
                None         => {
                    let created_vector = vec![val.clone()];
                    base_data.keys.insert(name.to_string(), created_vector);
                },
            }
        }   
    }

    Ok(base_data)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_data = macerate_objects(&args[1]);

    for (key, vals) in base_data.unwrap().keys {
        println!("{} - {:?}", key, vals);
    }
}