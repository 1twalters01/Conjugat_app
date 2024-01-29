use std::collections::BTreeMap;
use crate::data_types::json_data::JsonData;
use crate::helper_functions::file_operations::{append_file, create_file, delete_file};


// Don't type check with the compiler
pub fn save_data_to_file<T>(data: &T, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

// Check with the compiler that the data input is a vector
pub fn save_data_vec_to_file<T>(data: &Vec<T>, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

// Check with the compiler that the data input is a Vec<JsonData>
pub fn save_json_data_vec_to_file(data: &Vec<JsonData>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

// Check with the compiler that the data input is a vec<BTreeMap> with any input types
fn save_map_vec_to_file<T, Y>(data: &Vec<BTreeMap<T, Y>>, file_path: &str)
where T: serde::Serialize, Y: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

// Check with the compiler that the data input is a Vec<BTreeMap<String, i64>>
fn save_string_i64_map_vec_to_file(string_i64_map_vec: &Vec<BTreeMap<String, i64>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(string_i64_map_vec).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

// Check with the compiler that the data input is a Vec<BTreeMap> with identical param types
fn save_identical_param_map_vec_to_file<T>(data: &Vec<BTreeMap<T, T>>, file_path: &str)
where T: serde::Serialize {
    let serialized_data: String = serde_json::to_string_pretty(data).unwrap();
    delete_file(file_path);
    let mut file = create_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}


