use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    fs::{self, OpenOptions, File},
    io::{self, Error, ErrorKind, Read, Write},
    result,
};

use crate::data_types::{
    json_data::JsonData,
    field::{
        Field,
        FieldOptions,
    },
    field_options::{
        LanguageField,
        GroupField,
        ModelField,
        EndingField,
        BaseField,
        TenseField,
        TenseSubfield,
        SubjectField,
        AuxiliaryField,
        ConjugateField,
        ConjugationField,
        SentenceField,
    },
};




// pub async fn async_scrape_html_from_url(url: String) -> String {
//     let mut content: String = String::new();
//     let response: String = reqwest::get(url).await.unwrap().text().await.unwrap();
//     content.push_str(response.as_str());
//     return content
// }

// pub fn scrape_html_from_url(url: &str) -> String {
//     let mut content: String = String::new();
//     let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
//     content.push_str(response.as_str());
//     return content
// }

pub fn read_html_from_file(file_path: &str) -> String {
    let mut content: String = String::new();
    let mut file: File = open_file(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    // append_file(&mut file, &content);
    return content
}

// pub fn read_data_from_file(file_path: &str) -> Vec<JsonData> {
//     let content: String = read_html_from_file(file_path);
//     let data: Vec<JsonData> = serde_json::from_str(content.as_str()).unwrap();
//     return data;
// }



