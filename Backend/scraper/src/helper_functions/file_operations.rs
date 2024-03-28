use std::{
    fs::{self, File, OpenOptions},
    io::{self, Error, ErrorKind, Read, Write},
    result,
};

pub fn open_file(file_path: &str) -> Result<File, io::Error> {
    let file_result = OpenOptions::new().write(true).read(true).open(file_path);

    match file_result {
        Ok(file) => return Ok(file),
        Err(err) => return Err(err),
    };
}

pub fn open_or_create_file(file_path: &str) -> Result<File, io::Error> {
    let file_result = OpenOptions::new().write(true).read(true).open(file_path);

    match file_result {
        Ok(file) => return Ok(file),
        Err(error) => match error.kind() {
            // If file not found then create the file else recoverable error
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => return Ok(file),
                Err(e) => return Err(e),
            },

            other_error_kind => {
                // Make better error message
                let msg = "Problem opening the file";
                return Err(Error::new(other_error_kind, msg));
            }
        },
    };
}

pub fn create_file(file_path: &str) -> Result<File, io::Error> {
    // If file not found then create the file else recoverable error
    match File::create(file_path) {
        Ok(file) => return Ok(file),
        Err(e) => return Err(e),
    };
}

pub fn write_file(file: &mut File, content: &String) -> result::Result<(), io::Error> {
    match file.write_all(content.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    };
}

pub fn append_file(file: &mut File, content: &String) -> result::Result<(), io::Error> {
    let mut old_content: String = String::new();
    file.read_to_string(&mut old_content).unwrap();
    let new_content: String = old_content + content;
    println!("{}", new_content);
    match file.write_all(&new_content.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    };
}

pub fn delete_file(file_path: &str) -> result::Result<(), io::Error> {
    match fs::remove_file(file_path) {
        Ok(()) => return Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {} // return Ok(()),
            other_error_kind => return Err(Error::new(other_error_kind, "Error")),
        },
    };

    return Ok(());
}
