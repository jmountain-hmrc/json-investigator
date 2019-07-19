#[macro_use]
extern crate serde_json;

use std::fs::File;
use serde_json::Value;
use std::fs;
use std::io::{Write, BufReader, BufRead, Error};
use std::iter::Sum;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn findAveragesOfNumberIntersections(key: &String, vals: &Vec<Value>) {
    let mut num_tb = Vec::new();
    for value in vals {
        if value.is_number() && value.is_i64() {
            num_tb.push(value.as_i64().unwrap())
        }
    }

    if num_tb.len() > 0 {
        println!("Average of key: {} - {}", key, (num_tb.iter().sum::<i64>() as f64) / (num_tb.iter().len() as f64))
    }
}

fn findMedianOfNumberIntersections(key: &String, vals: &Vec<Value>) {
    let mut num_tb = Vec::new();
    for value in vals {
        if value.is_number() && value.is_i64() {
            num_tb.push(value.as_i64().unwrap())
        }
    }

    if num_tb.len() > 0 {
        num_tb.sort();

        println!("Median of key: {} - {}", key, num_tb[(num_tb.len() / 2)])
    }
}

fn findUniqueKeys(key: &String, vals: &Vec<Value>) {
    if vals.len() == 1 {
        println!("Key unique amongst objects: {}", key)
    }
}

fn findKeyWithAllSame(key: &String, vals: &Vec<Value>) {
    let mut hash_set = HashSet::new();
    for value in vals {
        hash_set.insert(value.to_string());
    }

    if hash_set.len() == 1 {
        let index = hash_set.get(&vals[0].to_string());

        println!("Key has only one value across all objects - {}: {}", key, index.unwrap())
    }
} 

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_data = macerate_objects(&args[1]);

    for (key, vals) in base_data.unwrap().keys {
        println!("{} - {:?}", key, vals);

        findAveragesOfNumberIntersections(&key, &vals);
        findMedianOfNumberIntersections(&key, &vals);
        findUniqueKeys(&key, &vals);
        findKeyWithAllSame(&key, &vals);

        println!("");
    }
}