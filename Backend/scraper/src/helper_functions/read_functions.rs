use crate::helper_functions::file_operations::{open_file, open_or_create_file};
use std::{
    fs::File,
    io::Read,
};

pub fn read_file_to_string(file_path: &str) -> String {
    let mut content: String = String::new();

    let mut file: File = open_file(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();

    return content
}

pub fn read_file_to_string_or_none(file_path: &str) -> String {
    let mut content: String = String::new();

    let mut file: File = open_or_create_file(file_path).unwrap();
    match file.read_to_string(&mut content) {
        Ok(_) => return content,
        Err(_) => return String::new(),
    };
}

