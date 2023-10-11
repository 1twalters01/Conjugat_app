// Todo
use crate::data_types::JsonData::{
    JsonData,
    Field,
    FieldOptions,
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
    BaseField,
    TenseField,
    SubjectField,
    AuxiliaryField,
    ConjugateField,
    ConjugationField,
};

use crate::helper_functions::{
    create_json_data_vec,
    create_pool_connection,
    save_data_to_json_file,
    read_data_from_file,
    scrape_html_from_url,
};

use std::{
    collections::HashSet,
    result,
};




pub async fn run_conjugations_modules() {
    // get vectors for the languages, groups, endings, and models
    let languages_data = read_data_from_file("temp/json/languages/languages.json");
    let groups_data = read_data_from_file("temp/json/models/groups.json");
    let endings_data = read_data_from_file("temp/json/models/endings.json");
    let models_data = read_data_from_file("temp/json/models/models.json");

    let languages: Vec<&str> = extract_languages(languages_data);

    let verb_urls: Vec<Vec<&str>> = form_verb_urls(languages);
}

fn extract_languages(languages_data: Vec<JsonData>) -> Vec<&'static str> {
    let mut languages: Vec<&str> = Vec::new();
    for language_data in languages_data {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            languages.push(language);
        }
    }

    return languages;
}



fn form_verb_url(languages) {
    for language in languages {
        //
    }
}



fn form_conjugation_url(language: &str, verb: &str) -> String {
    return String::from("https::/conjugator.reverso.net/conjugation-") + language + "-verb-" + verb + ".html";
}
