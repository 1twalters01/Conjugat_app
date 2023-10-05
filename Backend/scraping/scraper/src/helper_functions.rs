
use sqlx::{postgres::PgPoolOptions};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs::{self, OpenOptions, File};
use std::io::{self, ErrorKind, Read, Write};

pub fn open_file(file_path: &str) -> Result<File, io::Error> {
    let file_result = OpenOptions::new().write(true).read(true).open(file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If file not found then create the file else recoverable error
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => file,
                Err(e) => return Err(e),
            },
            
            other_error_kind => {
                // Make better error message
                let msg = "Problem opening the file";
                Err::new(other_error_kind, msg),
        }
    };

    return file
}

pub fn append_file(file: &mut File, content: String) {
    let mut old_content: String = String::new();
    let new_content: String = old_content + &content;
    // let check: () = file.write_all(new_content.as_bytes()).unwrap();
    file.write_all(new_content.as_bytes()).unwrap();
}


pub async fn async_scrape_html_from_url(url: &str) -> String {
    let mut content: String = String::new();
    let response: String = reqwest::get(url).await.unwrap().text().await.unwrap();
    content.push_str(response.as_str());
    return content
}

pub fn scrape_html_from_url(url: &str) -> String {
    let mut content: String = String::new();
    let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    content.push_str(response.as_str());
    return content
}

pub fn read_html_from_file(file_path: &str) -> string {
    let mut content: String = String::new();
    let mut file: File = open_file(file_path);
    file.read_to_string(&mut content);
    append_file(&mut file, content);
    return content
}

pub fn save_data_to_json_file(data:&Vec<JsonData>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(&data).unwrap();
    fs::remove_file(file_path);
    let mut file: File = open_file(file_path);
    append_file(&mut file, serialized_data
}

// Typing of pool is unknown atm so leaving it until I have a compiler
pub async fn create_pool_connection() -> {
    let pgusername: String = env::var("PG_USERNAME").unwrap();
    let pgpassword: String = env::var("PG_PASSWORD").unwrap();
    let pgdbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = String::from("postgres://") + pgusername.as_str() + ":"
        + pgpassword.as_str() + "@localhost:5432/" + pgdbname.as_str();

    // Create connection pool 
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await.unwrap();
  
    return pool
}

