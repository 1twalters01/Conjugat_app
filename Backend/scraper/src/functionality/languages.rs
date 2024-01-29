// Todo
use crate::data_types::{
    field::FieldOptions::LanguageField,
    json_data::{
        JsonData,
        create_json_data_vec_from_vec_vec_string,
    },
};

use crate::helper_functions::{
    save_functions::save_json_data_vec_to_file,
    postgres_functions::save_data_to_postgres,
};

use std::{
    collections::HashSet,
    result,
};



pub async fn run_languages_module(languages: Vec<String>) {
    match is_vector_valid(&languages) {
        Ok(res) => if res == false {panic!("invalid language vector")},
        Err(err) => panic!("{}", err),
    };

    let language_data_vec_vec: Vec<Vec<String>> = form_vec_vec(languages);

    let languages_data: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(language_data_vec_vec, LanguageField); 
    let file_path: &str = "temp/json/languages/languages.json";
    save_json_data_vec_to_file(&languages_data, file_path);

    println!("language data: {:#?}", languages_data);
    save_data_to_postgres(&languages_data);
}


// Improve this function
pub(crate) fn is_vector_valid(vector: &Vec<String>) -> result::Result<bool, &str> {
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

