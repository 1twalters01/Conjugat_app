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
        SubjectField,
        AuxiliaryField,
        ConjugateField,
        ConjugationField,
        SentenceField,
    },
};

pub fn open_file(file_path: &str) -> result::Result<File, io::Error> {
    let file_result = OpenOptions::new().write(true).read(true).open(file_path);

    let file = match file_result {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            // If file not found then create the file else recoverable error
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => Ok(file),
                Err(e) => return Err(e),
            },
            
            other_error_kind => {
                // Make better error message
                let msg = "Problem opening the file";
                Err(Error::new(other_error_kind, msg))
            },
        },
    };

    return file;
}

pub fn append_file(file: &mut File, content: &String) {
    let old_content: String = String::new();
    let new_content: String = old_content + content;
    // let check: () = file.write_all(new_content.as_bytes()).unwrap();
    file.write_all(new_content.as_bytes()).unwrap();
}


pub async fn async_scrape_html_from_url(url: String) -> String {
    let mut content: String = String::new();
    let response: String = reqwest::get(url).await.unwrap().text().await.unwrap();
    content.push_str(response.as_str());
    return content
}

pub fn scrape_html_from_url(url: &str) -> String {
    let mut content: String = String::new();
    let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    content.push_str(response.as_str());
    return content
}

pub fn read_html_from_file(file_path: &str) -> String {
    let mut content: String = String::new();
    let mut file: File = open_file(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    append_file(&mut file, &content);
    return content
}

// pub fn read_data_from_file(file_path: &str) -> Vec<JsonData> {
//     let content: String = read_html_from_file(file_path);
//     let data: Vec<JsonData> = serde_json::from_str(content.as_str()).unwrap();
//     return data;
// }

pub fn create_json_data_vec(data_vec_vec: Vec<Vec<String>>, field_type: FieldOptions) -> Vec<JsonData> {
    let mut json_data: Vec<JsonData> = Vec::new();
    let mut primary_key: i64 = 0;
   

    for (_index2, data) in data_vec_vec.into_iter().enumerate() {
        primary_key = primary_key + 1;

        let field: Field = match field_type {
            FieldOptions::LanguageField => {
                let language_field = LanguageField {
                    language: data[0].clone(),
                };
                Field::LanguageField(language_field)
            },

            FieldOptions::GroupField => {
                let group_field = GroupField {
                    language: data[0].clone(),
                    group: data[1].clone(),
                };
                Field::GroupField(group_field)
            },

            FieldOptions::EndingField => {
                let ending_field = EndingField {
                    group: data[0].clone(),
                    ending: data[1].clone(),
                };
                Field::EndingField(ending_field)
            },

            FieldOptions:: ModelField => {
                let model_field = ModelField {
                    ending: data[0].clone(),
                    model: data[1].clone(),
                };
                Field::ModelField(model_field)
            },

            FieldOptions::BaseField => {
                let base_field = BaseField {
                    rank: data[0].parse::<i64>().unwrap(),
                    language: data[1].clone(),
                    base: data[2].clone(),
                };
                Field::BaseField(base_field)
            },

            FieldOptions::TenseField => {
                let tense_field = TenseField {
                    language: data[0].clone(),
                    tense: data[1].clone(),
                };
                Field::TenseField(tense_field)
            },

            FieldOptions::SubjectField => {
                let subject_field = SubjectField {
                    language: data[0].clone(),
                    subject: data[1].clone(),
                };
                Field::SubjectField(subject_field)
            },

            FieldOptions::AuxiliaryField => {
                let auxiliary_field = AuxiliaryField {
                    language: data[0].clone(),
                    auxiliary: data[1].clone(),
                };
                Field::AuxiliaryField(auxiliary_field)
            },

            FieldOptions::ConjugateField => {
                let conjugate_field = ConjugateField {
                    base: data[0].clone(),
                    conjugate: data[1].clone(),
                    model: data[2].clone(),
                };
                Field::ConjugateField(conjugate_field)
            },

            // Need a different top type to Vec<Vec<&str>>
            FieldOptions::ConjugationField => {
                let conjugation_field = ConjugationField {
                    rank: data[0].parse::<i64>().unwrap(),
                    tense: data[1].clone(),
                    subject: data[2].clone(),
                    auxiliary: data[3].clone(),
                    conjugate: data[4].clone(),
                };
                Field::ConjugationField(conjugation_field)
            },

            // Need a different top type to Vec<Vec<&str>>
            FieldOptions::SentenceField => {
                let sentence_field = SentenceField {
                    rank: data[0].parse::<i64>().unwrap(),
                    conjugation: data[1].clone(),
                    sentence: data[2].clone(),
                    char_length: data[3].parse::<i64>().unwrap(),
                    char_start: data[4].parse::<i64>().unwrap(),
                };
                Field::SentenceField(sentence_field)
            },
        };

        let target_data = JsonData {
            pk: primary_key,
            fields: field,
            ..JsonData::default(&field_type)
        };

        json_data.push(target_data);
    }

    return json_data;
}


pub fn save_data_to_json_file(data:&Vec<JsonData>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(&data).unwrap();
    fs::remove_file(file_path).unwrap();
    let mut file = open_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}

pub async fn create_pool_connection() -> Pool<Postgres> {
    let pgusername: String = env::var("PG_USERNAME").unwrap();
    let pgpassword: String = env::var("PG_PASSWORD").unwrap();
    let pgdbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = String::from("postgres://") + pgusername.as_str() + ":"
        + pgpassword.as_str() + "@localhost:5432/" + pgdbname.as_str();

    // Create connection pool 
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await.unwrap();
  
    return pool
}

