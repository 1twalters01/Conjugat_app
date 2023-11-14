// Todo
use crate::data_types::{
    json_data::JsonData,
    field::{
        Field,
        FieldOptions,
    },
    field_options::LanguageField,
};

use crate::helper_functions::{
    create_json_data_vec,
    save_data_to_json_file,
    // create_pool_connection,
};

use std::{
    collections::HashSet,
    result,
};



pub async fn run_languages_module(languages: Vec<String>) {
    match is_vector_valid(&languages) {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    };

    let language_data_vec_vec: Vec<Vec<String>> = form_vec_vec(languages);

    let languages_data: Vec<JsonData> = create_json_data_vec(language_data_vec_vec, FieldOptions::LanguageField); 
    let file_path: &str = "temp/json/languages/languages.json";
    save_data_to_json_file(&languages_data, file_path);
    // save_language_data_to_postgres(&languages_data).await;
}


// Improve this function
fn is_vector_valid(vector: &Vec<String>) -> result::Result<bool, &str> {
    let hs: HashSet<String> = vector
        .iter()
        .cloned()
        .collect::<HashSet<String>>();
    
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


fn form_vec_vec(languages: Vec<String>) -> Vec<Vec<String>> {
    let mut languages_data_vec_vec: Vec<Vec<String>> = Vec::new();
    for language in languages {
        let data: Vec<String> = Vec::from([language]);
        languages_data_vec_vec.push(data);
    }
    
    return languages_data_vec_vec;
}


// async fn save_language_data_to_postgres(languages_data: &Vec<JsonData>) {
//     let pool = create_pool_connection().await;
//     
//     for language_data in languages_data {
//         if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
//             // if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_language (id, language) VALUES ($1, $2)")
//                 .bind(language_data.pk)
//                 .bind(language)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => {res},
//                 Err(_) => {
//                     let update_query = sqlx::query("UPDATE verbs_lanauge SET lanague=($1), WHERE id=($2)")
//                         .bind(language)
//                         .bind(language_data.pk)
//                         .execute(&pool).await;
//
//                     let update_result = match update_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     update_result
//                 }
//             };
//         } else {
//             panic!("non-ending in ending field");
//         };
//     }
// }
