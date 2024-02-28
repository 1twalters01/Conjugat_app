use std::{
    collections::BTreeMap,
    io::{Error, ErrorKind},
    fs::read_to_string,
};
use crate::data_types::json_data::JsonData;
use crate::helper_functions::file_operations::{append_file, create_file, delete_file, write_file};

use super::file_operations::open_file;


pub fn save_env(key: &str, value: &str) -> Result<(), Error> {
    let env_file_path = "./.env";
    // read in env file
    let env_content: String = read_to_string(env_file_path).unwrap();
    // split string by "\n"
    let split_env_content: Vec<&str> = env_content.split("\n").collect::<Vec<&str>>();
    // split by "="
    let mut env_tree: BTreeMap<&str, &str> = BTreeMap::new();
    let _ = split_env_content.into_iter()
        .map(|field| env_tree.insert(field.split_once("=").unwrap().0, field.split_once("=").unwrap().1));

    // if key == key then remove everything past = and append value
    match env_tree.contains_key(key) {
        true => {
            let mut new_env_content = String::new();
            env_tree.insert(key, value);
            for (key, value) in env_tree {
                let field = key.to_string() + "=" + value + "\n";
                new_env_content.push_str(field.as_str());
            }

            let mut env_file = open_file(env_file_path).unwrap();
            write_file(&mut env_file, &new_env_content).unwrap();

            return Ok(())
        },
        // if key not in env return error
        false => return Err(Error::new(ErrorKind::InvalidData, format!("Invalid env key"))),
    }
}

// Don't type check with the compiler
pub fn save_data_to_file<T>(data: &T, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a vector
pub fn save_data_vec_to_file<T>(data: &Vec<T>, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a Vec<JsonData>
pub fn save_json_data_vec_to_file(data: &Vec<JsonData>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a vec<BTreeMap> with any input types
pub fn save_map_vec_to_file<T, Y>(data: &Vec<BTreeMap<T, Y>>, file_path: &str)
where T: serde::Serialize, Y: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a Vec<BTreeMap<String, i64>>
pub fn save_string_i64_map_vec_to_file(string_i64_map_vec: &Vec<BTreeMap<String, i64>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(string_i64_map_vec).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a Vec<BTreeMap> with identical param types
pub fn save_identical_param_map_vec_to_file<T>(data: &Vec<BTreeMap<T, T>>, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

// Check with the compiler that the data input is a Vec<iVec<String>>
pub fn save_string_vec_vec_to_file(data: &Vec<Vec<String>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path).unwrap();
    let mut file = create_file(file_path).unwrap();
    write_file(&mut file, &serialized_data).unwrap();
}

