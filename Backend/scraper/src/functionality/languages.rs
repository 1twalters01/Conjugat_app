use crate::data_types::{
    field::{
        Field,
        FieldOptions,
    },
    json_data::{
        JsonData,
        create_json_data_vec_from_vec_vec_string,
    },
    field_options,
};

use crate::helper_functions::{
    // postgres_functions::save_data_to_postgres,
    save_functions::{
        save_json_data_vec_to_file,
        save_map_vec_to_file,
    },
};

use std::{
    collections::{
        HashSet,
        BTreeMap,
    },
    result::Result,
};



pub async fn run_languages_module(language_vec: Vec<String>) {
    // validate language vector
    match is_language_vector_valid(&language_vec) {
        Ok(res) => if res == false { panic!("invalid language vector") },
        Err(err) => panic!("{}", err),
    };

    // create json data vector for the languages
    let language_vec_vec: Vec<Vec<String>> = reform_language_vec_vec(language_vec);
    let language_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(language_vec_vec, FieldOptions::LanguageField); 

    // save json data vector
    let file_path: &str = "temp/json/languages/languages.json";
    save_json_data_vec_to_file(&language_json_data_vec, file_path);
    println!("language data: {:#?}", language_json_data_vec);

    // create language maps
    let language_pk_map_vec: Vec<BTreeMap<String, i64>> = get_language_pk_map_vec(&language_json_data_vec);
    save_map_vec_to_file(&language_pk_map_vec, "temp/json/languages/btreemaps/languages.json");

    // save language data to postgres
    // save_data_to_postgres(&language_json_data_vec);
}


// Improve this function
pub(crate) fn is_language_vector_valid(vector: &Vec<String>) -> Result<bool, &str> {
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


fn reform_language_vec_vec(languages: Vec<String>) -> Vec<Vec<String>> {
    let mut languages_data_vec_vec: Vec<Vec<String>> = Vec::new();
    for language in languages {
        let data: Vec<String> = Vec::from([language]);
        languages_data_vec_vec.push(data);
    }
    
    return languages_data_vec_vec;
}


fn get_language_pk_map_vec(language_data_vec: &Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    let mut language_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();

    for language_data in language_data_vec {
        if let Field::LanguageField(field_options::LanguageField { language }) = &language_data.fields {
            let mut language_pk_map: BTreeMap<String, i64> = BTreeMap::new();
            language_pk_map.insert(language.clone(), language_data.pk);
            language_pk_map_vec.push(language_pk_map);
        }
    }

    return language_pk_map_vec;
}

