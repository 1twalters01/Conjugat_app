#![allow(unused)]

mod crud;
mod save_language;
mod save_model;

mod generate_urls;
mod generate_word_list;
mod scrape_verb;
mod scrape_model;

use crate::save_language::run_languages_module;

use crate::generate_urls::generate_url_text_files;
use crate::generate_word_list::generate_word_list_files;
use crate::scrape_verb::scrape_verb;
use crate::scrape_model::scrape_model;

use dotenv::dotenv;
use save_model::run_model_module;


#[tokio::main]
async  fn main() {
    dotenv().ok();

    // Complete

    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English"];
    // run_languages_module(languages).await;

    //Incomplete
    run_model_module().await;

    // generate_url_text_files(languages.clone());
    // println!("url list has been generated");

    // generate_word_list_files(languages);
    // println!("word list has been generated");

    // scrape_model(languages);
    // println!("model scraping has been completed");

    // let infinitive = "accorder"; let language = "French";
    // scrape_verb(infinitive, language);
    // println!("verb scraping has been completed");
}


