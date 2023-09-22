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



#[derive(Debug, Serialize, Deserialize, Clone)]
struct JsonData {
    model: String,
    pk: i64,
    fields: Field,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Field {
    LanguageField(LanguageField),
    GroupField(GroupField),
    EndingField(EndingField),
    ModelField(ModelField),
}


#[derive(Serialize, Deserialize, Clone)]
enum FieldOptions {
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
struct LanguageField {
    language: String,
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



impl JsonData {
    fn default(field_type: FieldOptions) -> JsonData {
        match field_type {
            FieldOptions::LanguageField => {
                return JsonData {
                    model: "verbs.languages".to_string(),
                    pk:0,
                    fields: Field::default(FieldOptions::LanguageField),
                }
            },

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
            FieldOptions::LanguageField => {
                let language_field = LanguageField {
                    language: String::from(""),
                };
                return Field::LanguageField(language_field)
            },

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


    let (groups_dict, endings_groups_dict, models_endings_dict) = generate_languages_hashmaps(&all);
    // println!("groups_dict\n{:?}\n\nendings_groups_dict\n{:?}\n\nmodels_endings_dict\n{:?}\n\n", groups_dict, endings_groups_dict, models_endings_dict);


    let (groups_data, endings_data, models_data) = generate_vectors(&all, &groups_dict, &endings_groups_dict, &models_endings_dict);
    // println!("groups_data\n{:?}\n\nendings_data\n{:?}\n\nmodels_data\n{:?}\n\n", groups_data, endings_data, models_data);;


    generate_json_files(&groups_data, &endings_data, &models_data);
    save_to_postgres(&groups_data, &endings_data, &models_data).await;
}






fn read_language_json() -> (HashMap<String, i64>, Vec<String>) {
    let file_path = String::from("temp/json/languages/languages.json");
    let mut file = open_file(file_path);

    let mut language_data = String::from("");
    file.read_to_string(&mut language_data);

    let languages_json: Vec<JsonData> = serde_json::from_str(language_data.as_str()).unwrap();
    let mut language_hash: HashMap<String, i64> = HashMap::new(); 
    let mut languages: Vec<String> = Vec::new();

    for language_json in &languages_json {
        if let Field::LanguageField(LanguageField{ language }) = &language_json.fields {
            language_hash.insert(language.clone(), language_json.clone().pk);
            languages.push(language.clone());
        }; 
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
fn generate_languages_hashmaps(all: &Vec<Vec<[String; 3]>>) -> (Vec<BTreeMap<String, i64>>, Vec<BTreeMap<String, String>>,  Vec<BTreeMap<String, String>>) {
    let mut groups_dict: Vec<BTreeMap<String, i64>> = Vec::new();
    let mut endings_groups_dict: Vec<BTreeMap<String, String>> = Vec::new();
    let mut models_endings_dict: Vec<BTreeMap<String, String>> = Vec::new();

    for language_vec in all {
        let mut group_dict: BTreeMap<String, i64> = BTreeMap::new();
        let mut ending_group_dict: BTreeMap<String, String> = BTreeMap::new();
        let mut model_ending_dict: BTreeMap<String, String> = BTreeMap::new();
        
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





fn generate_vectors(all: &Vec<Vec<[String; 3]>>, groups_dict: &Vec<BTreeMap<String, i64>>, endings_groups_dict: &Vec<BTreeMap<String, String>>, models_endings_dict: &Vec<BTreeMap<String, String>>) -> (Vec<JsonData>, Vec<JsonData>, Vec<JsonData>) {
    let mut groups_data: Vec<JsonData> = Vec::new();
    let mut endings_data: Vec<JsonData> = Vec::new();
    let mut models_data: Vec<JsonData> = Vec::new();

    for (index, language_vec) in all.into_iter().enumerate() {

        // Group
        let mut swaped_groups_dict: BTreeMap<i64, String> = BTreeMap::new();

        for (key, value) in &groups_dict[index] {
            swaped_groups_dict.insert(value.clone(), key.clone());
        }

        for (key, value) in &swaped_groups_dict {
            let group_field = GroupField {
                language: (index + 1).to_string(), 
                group: value.to_string(),
            };

            let group_data = JsonData {
                fields: Field::GroupField(group_field),
                pk: *key,
                ..JsonData::default(FieldOptions::GroupField)
            };

            groups_data.push(group_data);
        }


        // Ending
        let mut swaped_endings_groups_vec: Vec<[String; 2]> = Vec::new();

        for (key, value) in &endings_groups_dict[index] {
            swaped_endings_groups_vec.push([value.clone(), key.clone()]);
        }

        swaped_endings_groups_vec.sort();
        
        for item in &swaped_endings_groups_vec {
            let ending_field = EndingField {
                group: groups_dict[index].get(&item[0]).unwrap().to_string(),
                ending: item[1].to_string(),
            };

            let ending_data = JsonData {
                fields: Field::EndingField(ending_field),
                ..JsonData::default(FieldOptions::EndingField)
            };

            endings_data.push(ending_data);
        }


        // Model
        for (key, value) in &models_endings_dict[index] {
            let model_field = ModelField {
                ending: endings_groups_dict[index].get(value).unwrap().to_string(),
                model: key.to_string(),
            };

            let model_data = JsonData {
                fields: Field::ModelField(model_field),
                ..JsonData::default(FieldOptions::ModelField)
            };

            models_data.push(model_data);
        }

    } 

    return (groups_data, endings_data, models_data);
}







fn generate_json_files(groups_data: &Vec<JsonData>, endings_data: &Vec<JsonData>, models_data: &Vec<JsonData>) {
    // groups
    let groups_json: String = serde_json::to_string_pretty(&groups_data).unwrap();

    let groups_file_path: String = "temp/json/models/groups.json".to_string();
    delete_file(groups_file_path.clone());

    let mut file: File = open_file(groups_file_path);
    append_file(&mut file, groups_json.clone());



    // endings
    let endings_json: String = serde_json::to_string_pretty(&endings_data).unwrap();

    let endings_file_path: String = "temp/json/models/endings.json".to_string();
    delete_file(endings_file_path.clone());

    let mut file: File = open_file(endings_file_path);
    append_file(&mut file, endings_json.clone());



    // models
    let models_json: String = serde_json::to_string_pretty(&models_data).unwrap();

    let models_file_path: String = "temp/json/models/models.json".to_string();
    delete_file(models_file_path.clone());

    let mut file: File = open_file(models_file_path);
    append_file(&mut file, models_json.clone());


}



async fn save_to_postgres(groups_data: &Vec<JsonData>, endings_data: &Vec<JsonData>, models_data: &Vec<JsonData>) {
    println!("test1");
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


    for group_data in groups_data {
        println!("{:?}, {:?}", group_data, group_data.pk);
        if let Field::GroupField(GroupField{language, group}) = &group_data.fields {

            //if unable to insert into table then update table else panic
            let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
                .bind(group_data.pk)
                .bind(language)
                .bind(group)
                .execute(&pool)
                .await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
                        .bind(language)
                        .bind(group)
                        .bind(group_data.pk)
                        .execute(&pool).await;

                    let rewrite_result = match rewrite_query {
                        Ok(res) => res,
                        Err(err) => panic!("Error: {:?}", err),
                    };
                    rewrite_result
                },
            };

        } else {
            panic!("non-group in group field");
        };
    }


    for ending_data in endings_data {
        if let Field::EndingField(EndingField { group, ending }) = &ending_data.fields {
            // if unable to insert into table then update table else panic
            let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
                .bind(ending_data.pk)
                .bind(group)
                .bind(ending)
                .execute(&pool).await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_ending SET group=($1), ending=($2), WHERE id=($3)")
                        .bind(group)
                        .bind(ending)
                        .bind(ending_data.pk)
                        .execute(&pool).await;

                    let rewrite_result = match rewrite_query {
                        Ok(res) => res,
                        Err(err) => panic!("Error: {:?}", err),
                    };
                    rewrite_result
                }
            };
        }
    }
    

    for model_data in endings_data {
        if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
        // if unable to insert into table then update table else panic
            let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
                .bind(model_data.pk)
                .bind(ending)
                .bind(model)
                .execute(&pool).await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
                        .bind(ending)
                        .bind(model)
                        .bind(model_data.pk)
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

}
