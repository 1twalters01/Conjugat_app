#![allow(unused)]

mod generate_urls;
mod generate_word_list;
mod scrape_verb;
mod scrape_model;

use crate::generate_urls::generate_url_text_files;
use crate::generate_word_list::generate_word_list_files;
use crate::scrape_verb::scrape_verb;
use crate::scrape_model::scrape_model;

mod crud;
use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};
use crate::crud::{append_file, delete_file, open_file};



fn main() {
    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English"];
    save_languages(languages);

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

use serde::{Deserialize, Serialize};
use serde_json::Result;

pub fn save_languages(languages: Vec<&str>) {

    #[derive(Debug, Serialize, Deserialize)]
    struct LanguageData {
        model: String,
        pk: usize,
        fields: Field,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Field {
        language: String,
    }
    
    impl LanguageData {
        fn default() -> LanguageData {
            LanguageData {
                model: "verbs.language".to_string(),
                pk: 1,
                fields:  Field::default(),
            }
        }
    }

    impl Field {
        fn default() -> Field {
            Field {
                language: "None".to_string(),
            }
        }
    }

    let mut languages_data: Vec<LanguageData> = Vec::new();

    for (index, language) in languages.into_iter().enumerate(){
        println!("{}, {}", index, language);

        let field = Field { language: language.to_string()};
        println!("{:?}", field);

        let language_data = LanguageData { pk: index + 1, fields: field, ..LanguageData::default() };
        println!("{:?}", language_data);
        
        languages_data.push(language_data);
    }

    println!("\n{:?}", languages_data);

    let languages_json = serde_json::to_string_pretty(&languages_data).unwrap();
    println!("\n{}", languages_json);
}

