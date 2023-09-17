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
    ending: String,
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
                    ending: "".to_string(),
                    model: "".to_string(),
                };
                return Field::ModelField(model_field)
            },
        } 
    }
}



pub async fn run_model_module() {
    let (language_hash, languages) = read_language_json();
    // println!("language hash {:?}\n\nlanguages {:?}\n\n", language_hash, languages);
    
    // let (all, groups, endings, models) = scrape_html(&languages);
    let (all) = scrape_html(&languages);
    let (groups, endings, models) = split_vec(&all);
    // println!("all\n{:?}\n\nmodels\n{:?}\n\ngroups\n{:?}\n\nendings{:?}\n\n", all, models, groups, endings);

    let (groups_data, endings_data, models_data) = generate_vectors(&all);

    let (groups_dict, endings_groups_dict, models_endings_dict) = generate_languages_hashmaps(&all);
    // println!("groups_dict\n{:?}\n\nendings_groups_dict\n{:?}\n\nmodels_endings_dict\n{:?}\n\n", groups_dict, endings_groups_dict, models_endings_dict)
    println!("{:?}", groups_dict);
    
}


fn generate_vectors(all: &Vec<Vec<[String; 3]>>) -> (Vec<JsonData>, Vec<JsonData>, Vec<JsonData>) {
    let mut groups_data: Vec<JsonData> = Vec::new();
    let mut endings_data: Vec<JsonData> = Vec::new();
    let mut models_data: Vec<JsonData> = Vec::new();

    // model_hash.insert(item[0].clone());
    // group_hash.insert(item[1].clone());
    // ending_hash.insert(item[2].clone());

    for (index, language_vec) in all.into_iter().enumerate() {
        for item in  language_vec {
            //group
            // Create hashmap, if item[1] not in hashmap then add to hashmap and do the following
            let group_field = GroupField {
                language: index.to_string(), 
                group: item[1].clone()
            };

            let group_data = JsonData {
                fields: Field::GroupField(group_field),
                ..JsonData::default(FieldOptions::GroupField)
            };

            groups_data.push(group_data);

            
            // ending
            let ending_field = EndingField {
                group: String::from(""),
                ending: item[2].clone()
            };
            
            let ending_data = JsonData {
                fields: Field::EndingField(ending_field),
                ..JsonData::default(FieldOptions::EndingField)
            };

            endings_data.push(ending_data);


            // model
            let model_field = ModelField {
                ending: String::from(""),
                model: item[0].clone()
            };

            let model_data = JsonData {
                    fields: Field::ModelField(model_field),
                    ..JsonData::default(FieldOptions::EndingField)
            };

            // item[0]
            // item[1]
            // item[2]
        }
    } 

    return (groups_data, endings_data, models_data);
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

    return (language_hash, languages)
}



// todo!("Get a vec of the different groups (e.g. er, ir, re in french)");
// Vec<Vec<String>> as [<ar, er, ir>, <ar, er, ir>, <are, ere, ire>, <er, ir, re>, <>] for groups, endings, models
//fn scrape_html(languages: &Vec<String>) -> (Vec<Vec<[String; 3]>>, Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
fn scrape_html(languages: &Vec<String>) -> (Vec<Vec<[String; 3]>>) {

    let mut all_plural: Vec<Vec<[String; 3]>> = Vec::new(); // change final array to array [String, String, String]
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut endings: Vec<Vec<String>> = Vec::new();
    let mut models: Vec<Vec<String>> = Vec::new();


    for language in languages {
        let url = "https://conjugator.reverso.net/conjugation-rules-model-".to_string() + language + ".html";


        let mut all: Vec<[String; 3]> = Vec::new();
        let mut group: Vec<String> = Vec::new();
        let mut ending: Vec<String> = Vec::new();
        let mut model: Vec<String> = Vec::new();

        let mut all_hash: HashSet<[String; 3]> = HashSet::new();
        let mut group_hash: HashSet<String> = HashSet::new();
        let mut ending_hash: HashSet<String> = HashSet::new();
        let mut model_hash: HashSet<String> = HashSet::new();


        let mut content: String = String::new();
        let file_path: String = "temp/models/".to_string() + language + ".txt";
        let mut file: File = open_file(file_path);

        file.read_to_string(&mut content);





        let section_container = scraper::Selector::parse("div.model-contents").unwrap();

        let all_selector = scraper::Selector::parse("div.model-row").unwrap();
        let model_selector = scraper::Selector::parse("a[class=model-title-verb]").unwrap();
        let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
        let group_selector = scraper::Selector::parse("p[class=group]").unwrap();


        let document = scraper::Html::parse_document(&content);

        for section in document.select(&section_container) {
            
            for all_scraped in section.select(&all_selector) {
                let all_div = all_scraped.text().collect::<Vec<_>>();
                // let model_content = all_div[0].to_string();
                // let group_content = all_div[1].to_string();

                let model_a = all_scraped.select(&model_selector).next().unwrap().text().collect::<Vec<_>>();
                let group_p = all_scraped.select(&group_selector).next().unwrap().text().collect::<Vec<_>>();
                let ending_p = all_scraped.select(&ending_selector).next().unwrap().text().collect::<Vec<_>>();

                let mut model_content = String::new();
                let mut group_content = String::new();
                let mut ending_content = String::new();
                let mut all_content = [String::new(), String::new(), String::new()];

                if (model_a.is_empty() == false) {
                    model_content = model_a[0].trim().to_string();
                    all_content[0] = model_a[0].trim().to_string();
                } else {
                    model_content = String::from("-");
                    all_content[0] = String::from("-");
                }
 
                if (group_p.is_empty() == false) {
                    group_content = group_p[0].trim().to_string();
                    all_content[1] = group_p[0].trim().to_string();
                } else {
                    group_content = String::from("-");
                    all_content[1] = String::from("-");
                }

                if (ending_p.is_empty() == false) {
                    ending_content = ending_p[0].trim().to_string();
                    all_content[2] = ending_p[0].trim().to_string();
                } else {
                    ending_content = String::from("-");
                    all_content[2] = String::from("-");
                }
 
                all_hash.insert(all_content);
                model_hash.insert(model_content);
                group_hash.insert(group_content);
                ending_hash.insert(ending_content);
            }

            all = Vec::from_iter(all_hash.clone().into_iter());
            all.sort();

            model = Vec::from_iter(model_hash.clone().into_iter());
            model.sort();


            group = Vec::from_iter(group_hash.clone().into_iter());
            group.sort();

            ending = Vec::from_iter(ending_hash.clone().into_iter());
            ending.sort();

        }
        
        all_plural.push(all);
        models.push(model);
        groups.push(group);
        endings.push(ending);
    }

    // return (all_plural, groups, endings, models)
    return (all_plural)
}


fn split_vec(all: &Vec<Vec<[String; 3]>>) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut endings: Vec<Vec<String>> = Vec::new();
    let mut models: Vec<Vec<String>> = Vec::new();
    
    for language_vec in all {
        let mut group: Vec<String> = Vec::new();
        let mut ending: Vec<String> = Vec::new();
        let mut model: Vec<String> = Vec::new();
        
        let mut model_hash: HashSet<String> = HashSet::new();
        let mut ending_hash: HashSet<String> = HashSet::new();
        let mut group_hash: HashSet<String> = HashSet::new();

 
        for item in language_vec {
            model_hash.insert(item[0].clone());
            group_hash.insert(item[1].clone());
            ending_hash.insert(item[2].clone());
        }
        
        model = Vec::from_iter(model_hash.clone().into_iter());
        model.sort();


        group = Vec::from_iter(group_hash.clone().into_iter());
        group.sort();

        ending = Vec::from_iter(ending_hash.clone().into_iter());
        ending.sort();


        groups.push(group);
        endings.push(ending);
        models.push(model);
    }

    return (groups, endings, models);
}


// model = [0]; group = [1]; ending = [2];
// endings_groups_dict, models_endings_dict
fn generate_languages_hashmaps(all: &Vec<Vec<[String; 3]>>) -> (Vec<HashMap<String, i64>>, Vec<HashMap<String, String>>,  Vec<HashMap<String, String>>) {
    let mut groups_dict: Vec<HashMap<String, i64>> = Vec::new();
    let mut endings_groups_dict: Vec<HashMap<String, String>> = Vec::new();
    let mut models_endings_dict: Vec<HashMap<String, String>> = Vec::new();

    for language_vec in all {
        let mut group_dict: HashMap<String, i64> = HashMap::new();
        let mut ending_group_dict: HashMap<String, String> = HashMap::new();
        let mut model_ending_dict: HashMap<String, String> = HashMap::new();
        
        for item in language_vec {
            let model = item[0].clone();
            let group = item[1].clone();
            let ending = item[2].clone();
            
            if (group_dict.contains_key(&group) == false) {
                group_dict.insert(group.clone(), GROUP_PK_COUNTER.fetch_add(1, Ordering::SeqCst).clone()); 
            }
            ending_group_dict.insert(ending.clone(), group);
            model_ending_dict.insert(model, ending);
        }
        
        groups_dict.push(group_dict);
        endings_groups_dict.push(ending_group_dict);
        models_endings_dict.push(model_ending_dict);
    }

    return (groups_dict, endings_groups_dict, models_endings_dict);
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
