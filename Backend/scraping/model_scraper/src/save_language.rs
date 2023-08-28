use crate::{append_file, delete_file, open_file};
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sqlx::postgres::PgPoolOptions;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct LanguageData {
    model: String,
    pk: i64,
    fields: Field,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    // TODO: automatic incremental pk
}

impl Field {
    fn default() -> Field {
        Field {
            language: "None".to_string(),
        }
    }
    // TODO: ensure language is unique
}


pub fn run_languages_module(languages: Vec<&str>) {
    let languages_data = create_languages_vec(languages);
    generate_languages_json_file(&languages_data);
    save_languages_to_postgres(&languages_data);
}



fn create_languages_vec(languages: Vec<&str>) -> Vec<LanguageData> {
    let mut languages_data: Vec<LanguageData> = Vec::new();

    for (index, language) in languages.into_iter().enumerate(){
        let field = Field { language: language.to_string()};

        let language_data = LanguageData { pk: index.to_string().parse::<i64>().unwrap() + 1, fields: field, ..LanguageData::default() };
        languages_data.push(language_data);
    }

    return languages_data;
} 


fn generate_languages_json_file(languages_data: &Vec<LanguageData>) {
    let languages_json = serde_json::to_string_pretty(&languages_data).unwrap();

    let file_path: String = "temp/json/languages/languages.json".to_string();
    delete_file(file_path.clone());
    let mut file: File = open_file(file_path);
    append_file(&mut file, languages_json.clone());
    println!("{}", languages_json)
}


#[tokio::main]
async fn save_languages_to_postgres(languages_data: &Vec<LanguageData>) {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test.await").await.unwrap();


    for language_data in languages_data {
        let result = sqlx::query(
            "INSERT INTO $1 (pk, language) VALUES ($2, $3)")
            .bind(language_data.model.clone())
            .bind(language_data.pk)
            .bind(language_data.fields.language.clone())
            .execute(&pool).await;
    }


  
}
