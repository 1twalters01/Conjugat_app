use std::io;

use crate::functionality::{
    languages::run_languages_module,
    models::run_model_module,
};

pub async fn initialise_process() {
    println!("Enter the languages you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer: &str = buffer.trim();

    let languages_str: Vec<&str> = trimmed_buffer.split(", ").collect::<Vec<&str>>();
    let mut languages: Vec<String> = Vec::new();
    for language in languages_str {
        languages.push(language.to_string());
    }

    println!("{:?}", languages);

    if languages.is_empty() {
        languages = Vec::from([
        "Spanish".to_string(),
        "Portuguese".to_string(),
        "Italian".to_string(),
        "French".to_string(),
        "English".to_string()
        ]);
    }

    run_languages_module(languages).await;
    run_model_module().await;
}


pub fn continue_process() {
    println!("Enter the languages you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer = buffer.trim();

    let languages = trimmed_buffer.split(", ").collect::<Vec<&str>>();
    println!("{:?}", languages);




    // TODO
    println!("What words would you like to scrape?");
    buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer = buffer.trim();

    // Split by lanugae
    let infinitives = trimmed_buffer.split(",").collect::<Vec<&str>>();
    println!("{:?}", infinitives);

    println!("continue");
}


