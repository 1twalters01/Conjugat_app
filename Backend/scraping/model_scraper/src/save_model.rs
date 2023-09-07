use crate::crud::{append_file, delete_file, open_file};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sqlx::{postgres::PgPoolOptions, Row};
use std::{
    collections::{HashMap, HashSet},
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



#[derive(Serialize, Deserialize, Clone)]
struct JsonData {
    model: String,
    pk: i64,
    fields: Field,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LanguageData {
    model: String,
    pk: i64,
    fields: LanguageField,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct LanguageField {
    language: String,
}



#[derive(Serialize, Deserialize, Clone)]
enum Field {
    GroupField(GroupField),
    EndingField(EndingField),
    ModelField(ModelField),
}


#[derive(Serialize, Deserialize, Clone)]
enum FieldOptions {
    GroupField,
    EndingField,
    ModelField,
}



#[derive(Serialize, Deserialize, Clone)]
struct GroupField {
    language: String,
    group: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EndingField {
    group: String,
    ending: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ModelField {
    group: String,
    model: String,
}



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
                    group: "".to_string(),
                    model: "".to_string(),
                };
                return Field::ModelField(model_field)
            },
        } 
    }
}



pub async fn run_model_module() {
    let (language_hash, languages) = read_language_json();
    let (groups, endings, models) = scrape_html(&languages);

    // let group_data: Vec<JsonData> = create_group_vec(&languages);
    // let ending_data: Vec<JsonData> = create_ending_vec(&languages);
    // let model_data: Vec<JsonData> = create_model_vec(&languages);

    // generate_model_json_file(&data);
    // save_model_to_postgres(&data).await;
}



fn read_language_json() -> (HashMap<String, i64>, Vec<String>) {
    let file_path = String::from("temp/json/languages/languages.json");
    let mut file = open_file(file_path);

    let mut language_data = String::from("");
    file.read_to_string(&mut language_data);

    let language_json: Vec<LanguageData> = serde_json::from_str(language_data.as_str()).unwrap();
    let mut language_hash: HashMap<String, i64> = HashMap::new(); 
    let mut languages: Vec<String> = Vec::new();

    for language in &language_json {
        language_hash.insert(language.clone().fields.language, language.clone().pk);
        languages.push(language.clone().fields.language);
    }

    // println!(
    //     "{:?}, {:?}",
    //     &language_json[0].fields.language,
    //     language_hash.get(&language_json[0].fields.language).unwrap()
    // );

    return (language_hash, languages)
}



// todo!("Get a vec of the different groups (e.g. er, ir, re in french)");
// Vec<Vec<String>> as [<ar, er, ir>, <ar, er, ir>, <are, ere, ire>, <er, ir, re>, <>] for groups, endings, models
fn scrape_html(languages: &Vec<String>) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {

    let mut models: Vec<Vec<String>> = Vec::new();
    let mut endings: Vec<Vec<String>> = Vec::new();
    let mut groups: Vec<Vec<String>> = Vec::new();


    for language in languages {
        let url = "https://conjugator.reverso.net/conjugation-rules-model-".to_string() + language + ".html";



        let mut model: Vec<String> = Vec::new();
        let mut ending: Vec<String> = Vec::new();
        let mut group: Vec<String> = Vec::new();
        let mut group_hash: HashSet<String> = HashSet::new();


        let mut content: String = String::new();
        let file_path: String = "temp/models/".to_string() + language + ".txt";
        let mut file: File = open_file(file_path);
       
        // let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
        // content.push_str(response.as_str());
        // append_file(&mut file, content);

        file.read_to_string(&mut content);





        let section_container = scraper::Selector::parse("div.model-contents").unwrap();

        let model_selector = scraper::Selector::parse("a[class=model-title-verb]").unwrap();
        let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
        let group_selector = scraper::Selector::parse("p[class=group]").unwrap();


        let document = scraper::Html::parse_document(&content);

        for section in document.select(&section_container) {

            for model_scraped in section.select(&model_selector) {
                let model_a = model_scraped.text().collect::<Vec<_>>();
                let model_content = model_a[0].to_string();
                model.push(model_content);
            }

            for group_scraped in section.select(&group_selector) {
                let group_p = group_scraped.text().collect::<Vec<_>>();

                if group_p.is_empty() == false {
                    let group_content = group_p[0].to_string();
                    group_hash.insert(group_content);
                }
            }

            group = Vec::from_iter(group_hash.clone().into_iter());
            group.sort();

            for ending_scraped in section.select(&ending_selector) {
                let ending_p = ending_scraped.text().collect::<Vec<_>>();
                let ending_content = ending_p[0].trim().to_string();
                ending.push(ending_content);
            }
        }

        models.push(model);
        groups.push(group);
        endings.push(ending);

        // println!("{:?}", models);
    }
    println!("{:?}", groups.clone());

    return (groups, endings, models)
}




fn create_group_vec(languages: &Vec<&str>, language_hash: &HashMap<String, i64>, groups: Vec<String>) -> Vec<Vec<JsonData>> {
    let mut groups_data: Vec<Vec<JsonData>> = Vec::new();

    for language in languages {
        let field_language = language_hash
            .get(language.clone())
            .unwrap()
            .clone()
            .to_string();

        for group in &groups {
            let group_fields = Field::GroupField(GroupField {
                language: field_language.clone(),
                group: group.clone(),
            });

            let group_data = JsonData {
                fields: group_fields,
                ..JsonData::default(FieldOptions::GroupField)
            };
        }
    }

    return groups_data;
}





fn create_ending_vec(languages: &Vec<&str>) -> Vec<JsonData> {
    let mut endings_data: Vec<JsonData> = Vec::new();



    return endings_data
}

fn create_model_vec(languages: &Vec<&str>) -> Vec<JsonData> {
    let mut models_data: Vec<JsonData> = Vec::new();



    return models_data
}



fn generate_model_json_file(groups: &Vec<JsonData>, endings: &Vec<JsonData>, models: &Vec<JsonData>) {

}



async fn save_model_to_postgres(groups: &Vec<JsonData>, endings: &Vec<JsonData>, models: &Vec<JsonData>) {
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
