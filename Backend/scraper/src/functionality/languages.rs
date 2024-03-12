use crate::{
    data_types::{
        field::{Field, FieldOptions},
        json_data::{JsonData, create_json_data_vec_from_vec_vec_string},
        field_options,
    },
    helper_functions::{
        save_functions::{save_json_data_vec_to_file, save_btree_map_to_file},
        // postgres_functions::save_data_to_postgres,
    },
};

use std::{
    collections::{HashSet, BTreeMap},
    io::{Error, ErrorKind},
    result::Result,
};



pub async fn run_languages_module(language_vec: Vec<String>) {
    // create json data vector for the languages
    is_language_vector_valid(&language_vec).unwrap();
    let language_vec_vec: Vec<Vec<String>> = language_vec.into_iter().map(|language| Vec::from([language])).collect();
    let language_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(language_vec_vec, FieldOptions::LanguageField); 

    // save json data vector
    let json_data_file_path: &str = "temp/json/languages/languages.json";
    save_json_data_vec_to_file(&language_json_data_vec, json_data_file_path);

    // create language maps
    let language_pk_map: BTreeMap<String, i64> = get_language_pk_map_vec(&language_json_data_vec);
    let language_pk_map_file_path: &str = "temp/json/languages/btreemaps/languages.json";
    save_btree_map_to_file(&language_pk_map, language_pk_map_file_path);

    // save language data to postgres
    // save_data_to_postgres(&language_json_data_vec);
}


pub(crate) fn is_language_vector_valid(language_vec: &Vec<String>) -> Result<(), Error> {
    let language_hs: HashSet<String> = language_vec.iter().cloned().collect::<HashSet<String>>();
    
    if language_hs.len() != language_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Language vector has duplicated languages")
        );
        return Err(error);
    }

    if language_vec.contains(&String::new()) {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Language vector has null element(s)")
        );
        return Err(error);
    }

    let alphabetic_language_vec: Vec<String> = language_vec.into_iter()
        .filter_map(|language| if every(language.chars().map(|c| c.is_alphabetic())) {return Some(language.to_owned())} else {None}).collect();

    if alphabetic_language_vec.len() != language_vec.len() {
        let error: Error = Error::new(
            ErrorKind::InvalidData,
            format!("Language vector has languages with invalid characters")
        );
        return Err(error);
    }

    Ok(())
}


fn every<T, I>(v: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: std::ops::Not<Output = bool>,
{
    v.into_iter().all(|x| !!x)
}

fn get_language_pk_map_vec(language_data_vec: &Vec<JsonData>) -> BTreeMap<String, i64> {
    let mut language_pk_map: BTreeMap<String, i64> = BTreeMap::new();

    for language_data in language_data_vec {
        if let Field::LanguageField(field_options::LanguageField { language }) = &language_data.fields {
            language_pk_map.insert(language.clone(), language_data.pk);
        }
    }

    return language_pk_map;
}

