use crate::{append_file, delete_file, open_file};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sqlx::{postgres::PgPoolOptions, Row};
use std::{
    env,
    collections::HashSet,
    fs::File,
    result,
    sync::atomic::{AtomicI64, Ordering},
    thread
};



// atomic counter for auto increment
static PK_COUNTER: AtomicI64 = AtomicI64::new(1);



#[derive(Serialize, Deserialize, Clone)]
struct LanguageData {
    model: String,
    pk: i64,
    fields: Field,
}

#[derive(Serialize, Deserialize, Clone)]
struct Field {
    language: String,
}



impl LanguageData {
    fn default() -> LanguageData {
        LanguageData {
            model: "verbs.language".to_string(),
            pk: PK_COUNTER.fetch_add(1, Ordering::SeqCst),
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



pub async fn run_languages_module(languages: Vec<&str>) {
    match is_vector_valid(&languages) {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    };

    let languages_data: Vec<LanguageData> = create_languages_vec(languages);
    generate_languages_json_file(&languages_data);
    save_languages_to_postgres(&languages_data).await;
}



fn is_vector_valid<'a>(vector: &'a Vec<&'a str>) -> result::Result<bool, &str> {
    let hs: HashSet<&str> = vector
        .iter()
        .cloned()
        .collect::<HashSet<&str>>();

    if hs.len() != vector.len() {
        return Err("Vector has duplicated languages")
    }

    for elem in hs {
        if elem == "" {
            return Err("Vector has null element(s)")
        }
    }

    Ok(true)
}



fn create_languages_vec(languages: Vec<&str>) -> Vec<LanguageData> {
    let mut languages_data: Vec<LanguageData> = Vec::new();

    for (index, language) in languages.into_iter().enumerate(){
        let field = Field {
            language: language.to_string()
        };

        let language_data = LanguageData {
            fields: field,
            ..LanguageData::default()
        };

        languages_data.push(language_data);
    }

    return languages_data;
}



fn generate_languages_json_file(languages_data: &Vec<LanguageData>) {
    let languages_json: String = serde_json::to_string_pretty(&languages_data).unwrap();

    let file_path: String = "temp/json/languages/languages.json".to_string();
    delete_file(file_path.clone());

    let mut file: File = open_file(file_path);
    append_file(&mut file, languages_json.clone());
}



async fn save_languages_to_postgres(languages_data: &Vec<LanguageData>) {
    // Get values from .env file
    let pgusername: String = env::var("PG_USERNAME").unwrap();
    let pgpassword: String = env::var("PG_PASSWORD").unwrap();
    let pgdbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = String::from("postgres://") + pgusername.as_str() + ":"
        + pgpassword.as_str() + "@localhost:5432/" + pgdbname.as_str();

    // Create connection pool 
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await.unwrap();


    for language_data in languages_data {
        // if unable to insert into table then update table else panic
        let insert_query = sqlx::query("INSERT INTO verbs_language (id, language) VALUES ($1, $2)")
            .bind(language_data.pk)
            .bind(language_data.fields.language.clone())
            .execute(&pool).await;

        let insert_result = match insert_query {
            Ok(res) => res,
            Err(err) => {
                let rewrite_query = sqlx::query("UPDATE verbs_language SET language=($1) WHERE id=($2)")
                    .bind(language_data.fields.language.clone())
                    .bind(language_data.pk)
                    .execute(&pool).await;

                let rewrite_result = match rewrite_query {
                    Ok(res) => res,
                    Err(err) => panic!("Error: {:?}", err),
                };
                rewrite_result
            },
        };
    }
}
