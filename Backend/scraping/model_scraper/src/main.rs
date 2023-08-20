#![allow(unused)]

mod crud;
mod generate_urls;
mod generate_word_list;

use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};
use crate::generate_urls::generate_url_text_files;
use crate::generate_word_list::generate_word_list_files;

use crate::crud::{append_file, delete_file, open_file};



fn main() {
    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English"];

    // generate_url_text_files(languages.clone());
    // println!("url list has been generated");

    // generate_word_list_files(languages);
    // println!("word list has been generated");




    // extract data from: accorder
    let infinitive: &str = "accorder";
    let content: String = obtain_html_string(infinitive);
    // println!("{}", content);

    let tense_selector = scraper::Selector::parse("div[mobile-title]>p").unwrap();
    let tense_type_container = scraper::Selector::parse("div.word-wrap-row").unwrap();


    let document = scraper::Html::parse_document(&content);
    for (index, section) in document.select(&tense_type_container).enumerate() {
        let mut tense_h4:Vec<&str> = Vec::new();
        let mut spaced = String::new();
        let h4_selector = scraper::Selector::parse("div.word-wrap-title>h4").unwrap(); 

        for main_tense in section.select(&h4_selector) {
            tense_h4 = main_tense.text().collect::<Vec<_>>();
        }

        if tense_h4.len() == 0 {
            tense_h4 = vec![""]
        } else {
            spaced = tense_h4[0].clone().to_owned() + " ";
            tense_h4[0] = spaced.as_str();
        }

        for tense_scraped in section.select(&tense_selector) {
            let tense_p = tense_scraped.text().collect::<Vec<_>>();
            let tense = tense_h4.clone()[0].to_owned() + tense_p[0]; 
            println!("{:?}", tense);
        }
    }
}



fn obtain_html_string(infinitive: &str) -> String {
    let url: String = form_url(infinitive, "French");
    println!("{}", url);

    let file_path: String = "temp/".to_string() + infinitive + " html.txt";
    let mut file: File = open_file(file_path);


    let mut content: String = String::new();

    // let mut response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    // content.push_str(response.as_str());

    // append_file(&mut file, content);

    file.read_to_string(&mut content);

    return content;
}



fn form_url(infinitive: &str, language: &str) -> String {
    let url: String =  "https://conjugator.reverso.net/conjugation-".to_string() + language + "-verb-" + infinitive + ".html";
    return url;
}

