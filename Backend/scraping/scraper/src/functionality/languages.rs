// Todo
use data_types::{Field, JsonData};
use::helper_functions::{save_data_to_json_file, create_pool_connection};
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicI64, Ordering},
    thread,
};


// atomic counter for auto increment
static PK_COUNTER: AtomicI64 = AtomicI64::new(1);


pub async fn run_languages_module(languages: Vec<&str>) {
    match is_vector_valid(&languages) {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    };

    let languages_data: Vec<JsonData> = create_languages_vec(languages);
    let file_path: &str = "temp/json/languages/languages.json";
    save_data_to_json_file(&language_data, file_path)
    save_language_data_to_postgres(&languages_data).await;
}


// Improve this function
fn is_vector_valid<'a>(vector: &'a Vec<&'a str>) -> result::Result<bool, &'a str> {
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
    let mut languages_data: Vec<JsonData> = Vec::new();

    for (index, language) in languages.into_iter().enumerate(){
        let field = LanguageField {
            language: language.to_string()
        };

        let language_data = JsonData {
            pk: PK_COUNTER.fetch_add(1, Ordering::SeqCst),
            fields: Field::LanguageField(language_field),
            ..JsonData::default()
        };

        languages_data.push(language_data);
    }

    return languages_data;
}

async fn save_language_data_to_postgres(languages_data: &Vec<JsonData>) {
    pool = create_pool_connection()

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
