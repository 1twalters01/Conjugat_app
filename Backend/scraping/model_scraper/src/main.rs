#![allow(unused)]

mod generate_urls;
mod generate_word_list;
mod scrape_verb;
mod scrape_model;
mod save_language;

use crate::generate_urls::generate_url_text_files;
use crate::generate_word_list::generate_word_list_files;
use crate::scrape_verb::scrape_verb;
use crate::scrape_model::scrape_model;

mod crud;
use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};
use crate::crud::{append_file, delete_file, open_file};
use crate::save_language::run_languages_module;


#[tokio::main]
async  fn main() {
    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English", "Hindi", "Greek"];
    run_languages_module(languages).await;

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


