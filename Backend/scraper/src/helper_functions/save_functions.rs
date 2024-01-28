use crate::helper_functions::file_operations::{};

fn save_string_i64_map_vec(string_i64_map_vec: &Vec<BTreeMap<String, i64>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(&string_i64_map_vec).unwrap();
    fs::remove_file(file_path).unwrap();
    let mut file = open_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}


pub fn save_json_data_vec(data:&Vec<JsonData>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(&data).unwrap();
    fs::remove_file(file_path).unwrap();
    let mut file = open_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}
