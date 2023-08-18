use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};


pub fn open_file(file_path: String) -> File {
    let file_result = OpenOptions::new().write(true).read(true).open(file_path.clone());  //.unwrap();

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_error => {
                panic!("Problem opening the file");
            }
        }
    };

    return file
}


pub fn rewrite_file(file: &mut File, content: String) {
    let check: () = file.write_all(content.as_bytes()).unwrap();
}


pub fn append_file(file: &mut File, content: String) {
    let mut old_content: String = String::new();
    let new_content: String = old_content + &content + "\n";
    let check: () = file.write_all(new_content.as_bytes()).unwrap();    
}

pub fn delete_file(file_path: String) {
    fs::remove_file(file_path);
}

