use crate::crud::{append_file, delete_file, open_file};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sqlx::{postgres::PgPoolOptions, Row};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    fs::File,
    io::Read,
    result,
    sync::atomic::{AtomicI64, Ordering},
    thread
};


static GROUP_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static ENDING_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static MODEL_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static BASE_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static TENSE_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static SUBJECT_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static AUXILIARY_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static CONJUGATE_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static SENTENCE_PK_COUNTER: AtomicI64 = AtomicI64::new(1);



#[derive(Debug, Serialize, Deserialize, Clone)]
struct JsonData {
    model: String,
    pk: i64,
    fields: Field,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Field {
    GroupField(GroupField),
    EndingField(EndingField),
    ModelField(ModelField),
    BaseField(BaseField),
    TenseField(TenseField),
    SubjectField(SubjectField),
    AuxiliaryField(AuxiliaryField),
    ConjugateField(ConjugateField),
    SentenceField(SentenceField),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
enum FieldOptions {
    GroupField,
    EndingField,
    ModelField,
    BaseField,
    TenseField,
    SubjectField,
    AuxiliaryField,
    ConjugateField,
    SentenceField,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupField {
    language: String,
    group: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EndingField {
    group: String,
    ending: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModelField {
    ending: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BaseField {
    rank: i64,
    language: String,
    base: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TenseField {
    language: String,
    tense: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubjectField {
    language: String,
    subject: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuxiliaryField {
    language: String,
    auxiliary: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConjugateField {
    base: String,
    conjugate: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct SentenceField {
    rank: i64,
    tense: String,
    subject: String,
    auxiliary: String,
    conjugate: String
}


// BaseField, TenseField, SubjectField, AuxiliaryField, ConjugateField, SentenceField

impl JsonData {
    fn default(field_type: FieldOptions) -> JsonData {
        match field_type {
            FieldOptions::GroupField => {
                return JsonData {
                    model: "verbs.groups".to_string(),
                    pk: GROUP_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::GroupField),
                }
            },

            FieldOptions::EndingField => {
                return JsonData {
                    model: "verbs.endings".to_string(),
                    pk: ENDING_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::EndingField),
                }
            },
 
            FieldOptions::ModelField => {
                return JsonData {
                    model: "verbs.models".to_string(),
                    pk: MODEL_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::ModelField),
                }
            },

            FieldOptions::BaseField => {
                return JsonData {
                    model: "verbs.bases".to_string(),
                    pk: BASE_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::BaseField),
                }
            },

            FieldOptions::TenseField => {
                return JsonData {
                    model: "verbs.endings".to_string(),
                    pk: TENSE_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::TenseField),
                }
            },
 
            FieldOptions::SubjectField => {
                return JsonData {
                    model: "verbs.models".to_string(),
                    pk: SUBJECT_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::SubjectField),
                }
            },
 
            FieldOptions::AuxiliaryField => {
                return JsonData {
                    model: "verbs.models".to_string(),
                    pk: SUBJECT_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::AuxiliaryField),
                }
            },
 
            FieldOptions::ConjugateField => {
                return JsonData {
                    model: "verbs.models".to_string(),
                    pk: SUBJECT_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::ConjugateField),
                }
            },
 
            FieldOptions::SentenceField => {
                return JsonData {
                    model: "verbs.models".to_string(),
                    pk: SUBJECT_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                    fields: Field::default(FieldOptions::SentenceField),
                }
            },
        }
    }
}



impl Field {
    fn default(field_type: FieldOptions) -> Field {
        match field_type {
            FieldOptions::GroupField => {
                let group_field = GroupField {
                    language: String::from(""),
                    group: "".to_string(), 
                };
                return Field::GroupField(group_field)
            },

            FieldOptions::EndingField => {
                let ending_field = EndingField {
                    group: "".to_string(),
                    ending: "".to_string(),
                };
                return Field::EndingField(ending_field)
            },
           
            FieldOptions::ModelField => {
                let model_field = ModelField {
                    ending: "".to_string(),
                    model: "".to_string(),
                };
                return Field::ModelField(model_field)
            },

            FieldOptions::BaseField => {
                let base_field = BaseField {
                    rank: 0,
                    base: "".to_string(),
                    language: "".to_string(),
                };
                return Field::BaseField(base_field)
            },

            FieldOptions::TenseField => {
                let tense_field = TenseField {
                    tense: "".to_string(),
                    language: "".to_string(),
                };
                return Field::TenseField(tense_field)
            },

            FieldOptions::SubjectField => {
                let subject_field = SubjectField {
                    subject: "".to_string(),
                    language: "".to_string(),
                };
                return Field::SubjectField(subject_field)
            },

            FieldOptions::AuxiliaryField => {
                let auxiliary_field = AuxiliaryField {
                    auxiliary: "".to_string(),
                    language: "".to_string(),
                };
                return Field::AuxiliaryField(auxiliary_field)
            },

            FieldOptions::ConjugateField => {
                let conjugate_field = ConjugateField {
                    base: "".to_string(),
                    conjugate: "".to_string(),
                };
                return Field::ConjugateField(conjugate_field)
            },

            FieldOptions::SentenceField => {
                let sentence_field = SentenceField {
                    rank: 0,
                    tense: String::from(""),
                    subject: String::from(""),
                    auxiliary: String::from(""),
                    conjugate: String::from(""),
                };
                return Field::SentenceField(sentence_field)
            },

        } 
    }
}





pub async fn run_component_module() {
    let (group_hash, group) = read_group_json();
    // println!("group_hash{:?}\n\ngroup\n", group_hash, group);
    let (ending_hash, ending) = read_ending_json();
    // println!("ending_hash{:?}\n\nending\n", ending_hash, ending);
    let (model_hash, model) = read_model_json();
    // println!("model_hash{:?}\n\nmodel\n", model_hash, model);

    let () = scrape_html();



    let () = generate_vectors();

    generate_json_files(&bases_data);
    generate_json_files(&tenses_data);
    generate_json_files(&subjects_data);
    generate_json_files(&auxiliaries_data);
    generate_json_files(&conjugates_data);
    
    save_to_postgres().await;
}



fn read_group_json() -> (Vec<HashMap<String, String>>, Vec<JsonData>) {
	let file_path = String::from("temp/json/models/groups.json");
    let mut file = open_file(file_path);

    let mut group_data = String::from("");
    file.read_to_string(&mut group_data);

    let groups_json: Vec<JsonData> = serde_json::from_str(group_data.as_str()).unwrap();
    let mut group_hash: HashMap<String, String> = HashMap::new();
    let mut groups_vec_hash: Vec<HashMap<String, String> = Vec::new();
    let mut groups: Vec<String> = Vec::new();

    for (index, group_json) in &groups_json.into_iter().enumerate() {
        if let Field::GroupField(GroupField{ group, language }) = group_json.fields {
            // group_hash.insert(group + index.to_string(), language);
            group_hash.insert(group, language);
            groups_vec_hash[language.parse::<i64>()].insert(
            groups[language.parse::<i64>()].push(group);
        }; 
    }
    
    // for key in group_hash {
    //     if let Field::GroupField (GroupField{language, group}) = &group_data.fields {
    //         let language_index = language.parse::<i64>().unwrap() - 1;
    //         groups_vec_hash[language_index].insert(key);
    //     }
    // }

    return (groups_vec_hash, groups)
}



fn read_ending_json() -> (Vec<HashMap<String, String>>, Vec<JsonData>) {
	let file_path = String::from("temp/json/models/endings.json");
    let mut file = open_file(file_path);

    let mut ending_data = String::from("");
    file.read_to_string(&mut ending_data);

    let endings_json: Vec<JsonData> = serde_json::from_str(ending_data.as_str()).unwrap();
    let mut ending_hash: HashMap<String, String> = HashMap::new();
    let mut endings_vec_hash: Vec<HashMap<String, String> = Vec::new();
    let mut endings: Vec<String> = Vec::new();

    for (index, ending_json) in &endings_json.into_iter().enumerate() {
        if let Field::GroupField(GroupField{ ending, group }) = ending_json.fields {
            ending_hash.insert(ending, group);
            endings_vec_hash[group.parse::<i64>()].insert(
            endings[group.parse::<i64>()].push(ending);
        }; 
    }

    return (endings_vec_hash, endings)
}



fn read_model_json() -> (Vec<HashMap<String, String>>, Vec<JsonData>) {
	let file_path = String::from("temp/json/models/models.json");
    let mut file = open_file(file_path);

    let mut model_data = String::from("");
    file.read_to_string(&mut model_data);

    let models_json: Vec<JsonData> = serde_json::from_str(model_data.as_str()).unwrap();
    let mut model_hash: HashMap<String, String> = HashMap::new();
    let mut models_vec_hash: Vec<HashMap<String, String> = Vec::new();
    let mut models: Vec<String> = Vec::new();

    for (index, model_json) in &models_json.into_iter().enumerate() {
        if let Field::GroupField(GroupField{ model, ending }) = model_json.fields {
            model_hash.insert(ending, group);
            models_vec_hash[ending.parse::<i64>()].insert(
            models[ending.parse::<i64>()].push(model);
        }; 
    }

    return (endings_vec_hash, endings)
}


fn scrape_html() {}


fn generate_vectors() {}


fn generate_json_files(data: &Vec<JsonData>, file_path: String) {
	let json: String = serde_json::to_string_pretty(&data).unwrap();
    
    delete_file(file_path.clone());
    
    let mut file: File = open(file_path);
    append_file(&mut file, json.clone());
}


async fn save_to_postgres() {
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



}

