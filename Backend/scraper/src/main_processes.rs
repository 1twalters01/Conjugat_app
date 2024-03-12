use std::io;

use crate::functionality::{
    languages::run_languages_module,
    models::run_model_module,
    conjugations::run_conjugations_modules,
    resurrection::run_resurrection_module
};

pub async fn initialise_process() {
    println!("Enter the languages you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer: &str = buffer.trim();
    let mut languages_str: Vec<&str> = trimmed_buffer.split(",").collect::<Vec<&str>>();
    languages_str = languages_str.into_iter().map(|l| l.trim()).collect();
    let mut languages: Vec<String> = Vec::new();

    for language in languages_str {
        if language.len() > 0 {
            println!("{}", language);
            languages.push(language.to_string());
        }
    }

    if languages.is_empty() {
        languages = Vec::from([
        "Spanish".to_string(),
        "Portuguese".to_string(),
        "Italian".to_string(),
        "French".to_string(),
        "English".to_string()
        ]);
    }

    println!("languages: {:?}", languages);

    run_languages_module(languages).await;
    run_model_module().await;
    run_conjugations_modules().await;
}


pub fn continue_process() {
    run_resurrection_module();
}


