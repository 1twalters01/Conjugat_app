// Todo
use crate::data_types::{
    page_info::PageInfo,
    json_data::JsonData,
    field::{
        Field,
        // FieldOptions,
    },
    field_options::{
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
    }
};

use crate::helper_functions::{
    // create_json_data_vec,
    // create_pool_connection,
    // save_data_to_json_file,
    read_html_from_file,
    open_file,
    append_file,
};

use std::{
    // collections::{BTreeMap, HashSet},
    env,
    fs::{self, File},
    // result,
    // time::Duration,
    // thread,
};



pub async fn run_conjugations_modules() {
    // Read language data from file
    let language_content: String = read_html_from_file("temp/json/languages/languages.json");
    let (_language_data_vec, language_vec) = read_language_data_from_json_data(language_content.as_str());
    // println!("language_data_vec: {:#?}", _language_data_vec);

    // Read group data from file
    let group_content: String = read_html_from_file("temp/json/models/groups.json");
    let _group_data_vec: Vec<JsonData> = serde_json::from_str(group_content.as_str()).unwrap();
    // println!("group_data_vec: {:#?}", _group_data_vec);

    // Read ending data from file
    let ending_content: String = read_html_from_file("temp/json/models/endings.json");
    let _ending_data_vec: Vec<JsonData> = serde_json::from_str(ending_content.as_str()).unwrap();
    // println!("ending_data_vec: {:#?}", _ending_data_vec);

    // Read model data from file
    let model_content: String = read_html_from_file("temp/json/models/models.json");
    let _model_data_vec: Vec<JsonData> = serde_json::from_str(model_content.as_str()).unwrap();
    // println!("model_data_vec: {:#?}", _model_data_vec);

    // Get regular exponential back off & error 429 backoff
    let backoff: i64 = env::var("BACKOFF").unwrap().parse::<i64>().unwrap();
    let error_429_backoff: i64 = env::var("ERROR_429_BACKOFF").unwrap().parse::<i64>().unwrap();
    println!("backoff: {}, error 429 backoff: {}", backoff, error_429_backoff);

    // Fetch verb urls vector
    let verb_url_vec_vec: Vec<Vec<String>> = fetch_verb_url_vec_vec(language_vec, backoff, error_429_backoff);

    // Generate verb page information vector
    let verb_page_info_vec_vec: Vec<Vec<VerbPageInfo>> = generate_verb_page_info_vec_vec(verb_url_vec_vec, backoff, error_429_backoff);
    // println!("verb_page_info_vec: {:#?}", verb_page_info_vec);

    // Fetch data vectors
    let tense_vec_vec: Vec<JsonData> = extract_tense_json_data_vec(verb_page_info_vec_vec);
    let base_vec_vec: Vec<JsonData> = extract_base_json_data_vec(verb_page_info_vec_vec);
    let subject_vec_vec: Vec<JsonData> = extract_subject_json_data_vec(verb_page_info_vec_vec);
    let auxiliary_vec_vec: Vec<JsonData> = extract_auxiliary_json_data_vec(verb_page_info_vec_vec);
    let conjugate_vec_vec: Vec<JsonData> = extract_conjugate_json_data_vec(verb_page_info_vec_vec);
    let conjugation_vec_vec: Vec<JsonData> = extract_conjugation_json_data_vec(verb_page_info_vec_vec);

}


fn read_language_data_from_json_data(language_content: &str) -> (Vec<JsonData>, Vec<String>) {
    let language_data_vec: Vec<JsonData> = serde_json::from_str(language_content).unwrap();
    
    let mut language_vec: Vec<String> = Vec::new();
    for language_data in &language_data_vec {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            language_vec.push(language.clone());
        }
    }
    return (language_data_vec, language_vec);
}


fn fetch_verb_url_vec_vec(language_vec: Vec<String>, backoff: i64, error_429_backoff: i64) -> Vec<Vec<String>> {
// try to read urls_vec_vec otherwise scrape from reverso
    let verb_url_vec_vec_file_path: &str = "temp/json/conjugations/verb_urls.json";
    let verb_url_vec_content: String = read_html_from_file(verb_url_vec_vec_file_path);
    let verb_url_vec_vec_file_result: Result<Vec<Vec<String>>, serde_json::Error>
            = serde_json::from_str(verb_url_vec_content.as_str());


    let verb_url_vec_vec: Vec<Vec<String>> = match verb_url_vec_vec_file_result {
        Ok(verb_url_vec_vec) => verb_url_vec_vec,

        Err(_) => {
            let url_listing_vec_vec = generate_url_listing_vec_vec(&language_vec);
            println!("url_listing_vec_vec: {:?}", url_listing_vec_vec);

            let verb_vec_vec: Vec<Vec<String>> = scrape_url_listing_vec_vec(url_listing_vec_vec, backoff, error_429_backoff);

            let verb_url_vec_vec: Vec<Vec<String>> = generate_verb_url_vec_vec(verb_vec_vec, language_vec);
            save_verb_urls(&verb_url_vec_vec, "temp/json/verb_urls.json");

            return verb_url_vec_vec;
        }
    };

    println!("verb_url_vec_vec: {:?}", verb_url_vec_vec);
    return verb_url_vec_vec;
}


fn generate_url_listing_vec_vec(language_vec: &Vec<String>) -> Vec<Vec<String>> {
    let mut url_listing_vec_vec: Vec<Vec<String>> = Vec::new();

    let url_val_array: [i64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    for language in language_vec {
        let url_listing_vec: Vec<String> = url_val_array.map(|val|
            String::from("https://conjugator.reverso.net/index-")
            + language.as_str() + "-" + &(250*(val) + 1).to_string()
            + "-" + &(250*(val + 1)).to_string() + ".html")
            .to_vec();

        url_listing_vec_vec.push(url_listing_vec);
    }

    return url_listing_vec_vec;
}

fn scrape_url_listing_vec_vec(url_listing_vec: Vec<Vec<String>>, backoff: i64, error_429_backoff: i64) -> Vec<Vec<String>> {
    let mut verb_vec_vec: Vec<Vec<String>> = Vec::new();

    for url_vec in &url_listing_vec {
        let verb_vec: Vec<String> = Vec::new();
        todo!()
    }

    return verb_vec_vec;
}


fn generate_verb_url_vec_vec(verb_vec_vec: Vec<Vec<String>>, language_vec: Vec<String>) -> Vec<Vec<String>> {
    let verb_url_vec_vec: Vec<Vec<String>> = verb_vec_vec.into_iter().enumerate()
        .map(|(index, verb_vec)|
            verb_vec.into_iter().map(|verb| String::from("https://conjugator.reverso.net/conjugation-")
                + language_vec[index].as_str() + "-verb-" + verb.as_str() + "html")
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    return verb_url_vec_vec;
}


fn save_verb_urls(verb_urls_vec: &Vec<Vec<String>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(verb_urls_vec).unwrap();
    fs::remove_file(file_path).unwrap();
    let mut file = open_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);
}


fn generate_verb_page_info_vec_vec(verb_url_vec_vec: Vec<Vec<String>>, backoff: i64, error_429_backoff: i64) -> Vec<Vec<PageInfo>> {
    let mut verb_page_info_vec_vec: Vec<Vec<PageInfo>> = Vec::new();



    return verb_page_info_vec_vec;
}

// pub struct PageInfo {
//     pub metadata: PageMetadata,

//     pub tenses: Vec<Tenses>,
//     pub subjects: Vec<String>,
//     pub auxiliaries: Vec<String>,
//     pub conjugates: Vec<Vec<String>>,
// }

// struct PageMetadata {
//     pub language: String,
//     pub model: String,
//     pub base: String,
//     pub auxiliary: Vec<String>,
//     pub forms: Vec<String>,
//     pub similar_verbs: Vec<String>,
//     pub other_verbs: Vec<String>,
// }

// struct Tenses {
//     major: String,
//     minor: String,
// }

fn extract_base_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut base_data_vec_vec: Vec<Vec<String>> = Vec::new();
    let mut base_json_data_vec: Vec<JsonData> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let rank: String = verb_page_info.metadata.rank;
            let language: String = verb_page_info.metadata.language;
            let base: String = verb_page_info.metadata.base;

            let base_data_vec = Vec::from([rank, language, base]);

            if base_data_vec_vec.contains(base_data_vec) == 0 {
                base_data_vec_vec.push(base_data_vec);
            }
        }
    }
    
    let base_json_data_vec: Vec<JsonData> = create_json_data_vec(base_data_vec_vec, FieldOptions::BaseField);

    return base_json_data_vec;
}


fn extract_tense_json_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut tense_json_data_vec: Vec<JsonData> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language: String = verb_page_info.metadata.language;
            let tense_data: Tenses = verb_page_info.tenses

            // Split tense into mood and tense???
            // change tense fields to option(String) instead of String?
            let tense: Option(Vec<String>) = match tense_data {
                tense.major != None && tense.minor != None => {
                    return tense.major + " " + tense.minor.as_str();
                },
                
                tense.major != None && tense.minor == None => {

                },
                
                tense.major == None && tense.minor != None => {
                    return verb
                },
                
                tense.major == None && tense.minor == None => {
                
                }
            }
                
                
        }
    }



    return tense_json_data_vec;
}


fn extract_subject_json_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut subject_data_vec_vec: Vec<Vec<String>> = Vec::new();
    
    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language: String = verb_page_info.metadata.language;
            let subjects: Vec<String> = verb_page_info.subjects;

            for subject in subjects {
                let subject_data_vec: Vec<String> = String::from([language, subject]);
                if subject_data_vec_vec.contains(subject_data_vec) == 0 {
                    subject_data_vec_vec.push(subject_data_vec);
                }
                
            }
        }
    }

    let subject_json_data_vec: Vec<JsonData> = create_json_data_vec(subject_data_vec_vec, FieldOptions::SubjectField);

    return subject_json_data_vec;
}


fn extract_auxiliary_json_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut auxiliary_json_data_vec: Vec<JsonData> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language = verb_page_info.metadata.language;
            
        }
    }


    return auxiliary_json_data_vec 
}


fn extract_conjugate_json_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut conjugate_json_data_vec: Vec<JsonData> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            
        }
    }



    return conjugate_json_data_vec;
}


fn extract_conjugation_json_data_vec(verb_page_info_vec_vec: Vec<Vec<PageInfo>>) -> Vec<JsonData> {
    let mut conjugation_json_data_vec: Vec<JsonData> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            
        }
    }



    return conjugation_json_data_vec;
}



















// pub async fn run_conjugations_modules() {
//     // get vectors for the languages, groups, endings, and models
//     let languages_data = read_data_from_file("temp/json/languages/languages.json");
//     let groups_data = read_data_from_file("temp/json/models/groups.json");
//     let endings_data = read_data_from_file("temp/json/models/endings.json");
//     let models_data = read_data_from_file("temp/json/models/models.json");
//
//     let languages: Vec<&str> = extract_languages(languages_data);
//
//     let verb_urls_vec: Vec<Vec<&str>> = form_verb_urls(languages);
//     save_verb_urls(verb_urls_vec);
//
//     // Get exponential back off
//     let (exponential_backoff, error_429_backoff): i64 = read_exponential_backoff_values;
//     
//     for (language_id, verb_urls) in verb_urls_vec.into_iter().enumerate() {
//         for url in verb_urls {
//             // async_scrape_html_from_url(url: &str)
//             let mut content: String = String::new();
//             reqwest::get(url).await.unwrap().text().await.unwrap();
//             content.push_str(response.as_str());
//
//             let document = scraper::Html::parse_document(&content);
//             
//             // Scrape top bar of reverso website, aka model, auxiliaries and other forms
//             let top_section_container = scraper::Selector::parse("div.alternate-versions").unwrap();
//             let model_selector = scraper::Selector::parse("span[id=ch_lblModel]").unwrap();
//             let auxiliary_type_selector = scraper::Selector::parse("span[id=ch_lblAuxiliary]>a").unwrap();
//             let form_type_selector = scraper::Selector::parse("span[id=ch_lblAutreForm]>a").unwrap();
//         
//             let mut model: String = String::new();
//             let mut auxiliary_types: Vec<&str> = Vec::new();
//             let mut form_types: Vec<&str> = vec![infinitive];
//         
//             for mut section in document.select(&top_section_container) {
//         
//             for mut section in document.select(&top_section_container) {
//         
//             for mut section in document.select(&top_section_container) {
//         
//             for mut section in document.select(&top_section_container) {
//                 model = section.select(&model_selector).flat_map(|el| el.text().collect::<String>());
//                 println!("model: {}", model);
//         
//                 auxiliary_types = section.select(&auxiliary_type_selector).flat_map(|el| el.text()).collect::<Vec<&str>>();
//                 println!("auxiliary types: {:?}", auxiliary_types);
//         
//                 form_types.extend(section.select(&form_type_selector).flat_map(|el| el.text()).collect::<Vec<&str>>());
//                 println!("form types: {:?}", form_types);
//             }
//
//             // Create vec of alternate urls
//             let alt_urls: Vec<&str> = form_types.map(|el| String::from("https://conjugator.reverso.net/conjugation-") + languages[language_id] + el.replace(" ", "%20") + ".html");
//
//             // Scrape alternate urls
//             thread::sleep(Duration::from_millis(exponential_backoff));
//             let alt_content: Vec<String> = alt_urls.map(|url| async_scrape_html_from_url(url));
//
//             // Scrape lower section
//             //let tense_type_selector = scraper:;Selector::parse("div.word-wrap-title>h4").unwrap();
//             //let tense_main_selector = scraper::Selector::parse("").unwrap();
//             let tense_selector = scraper::Selector::parse("div[mobile-title]>p").unwrap();
//             let subject_selector = scraper::Selector::parse("i.graytxt").unwrap();
//             let auxiliary_selector = scraper::Selector::parse("span#ch_lblAuxiliary>a").unwrap();
//             let conjugate_selector = scraper::Selector::parse("i.verbtxt").unwrap();
//             // 
//             let language: &str = languages[language_id];
//             let base: &str = verb;
//             let base_rank: &str = ;
//             let tense: &str = ;
//             let subjuct: &str = ;
//             let auxiliary: &str = ;
//             let conjugate: &str = ;
//             let conjugation_rank: &str = ;
//             
//             let base_data_vec: Vec<&str> = vec![rank, language, base];
//             let tense_data_vec: Vec<&str> = vec![language, tense];
//             let subject_data_vec: Vec<&str> = vec![language, subject];
//             let auxiliary_data_vec: Vec<&str> = vec![language, auxiliary];
//             let conjugate_data_vec: Vec<&str> = vec![base, conjugate, model];
//             let conjugation_data_vec: Vec<&str> = vec![rank, tense, subject, auxiliary, conjugate];
//         }
//     }
//
//
//
//     let bases_data: Vec<JsonData> = create_json_data_vec(bases_data_vec_vec, FieldOptions::BaseField);
//     let tenses_data: Vec<JsonData> = create_json_data_vec(tenses_data_vec_vec, FieldOptions::TenseField);
//     let subjects_data: Vec<JsonData> = create_json_data_vec(subjects_data_vec_vec, FieldOptions::SubjectField);
//     let auxiliaries_data: Vec<JsonData> = create_json_data_vec(auxiliaries_data_vec_vec, FieldOptions::AuxiliaryField);
//     let conjugate_data: Vec<JsonData> = create_json_data_vec(conjugates_data_vec_vec, FieldOptions:ConjugateField);
//     let conjugations_data: Vec<JsonData> = create_json_data_vec(conjugations_data_vec_vec, FieldOptions::ConjugationField);
//
//
//     let bases_file_path: &str = "temp/json/conjugations/bases.json";
//     save_data_to_json_file(&bases_data, bases_file_path);
//     let tenses_file_path: &str = "temp/json/conjugations/tenses.json";
//     save_data_to_json_file(&tenses_data, tenses_file_path);
//     let subjects_file_path: &str = "temp/json/conjugations/subjects.json";
//     save_data_to_json_file(&subjects_data, subjects_file_path);
//     let auxiliaries_file_path: &str = "temp/json/conjugations/auxiliaries.json";
//     save_data_to_json_file(&auxiliaries_data, auxiliaries_file_path);
//     let conjugates_file_path: &str = "temp/json/conjugations/conjugates.json";
//     save_data_to_json_file(&conjugates_data, conjugates_file_path);
//     let conjugations_file_path: &str = "temp/json/conjugations/conjugations.json";
//     save_data_to_json_file(&conjugations_data, conjugations_file_path);
//
//     save_data_to_postgres(&bases_data, &tenses_data, &subjects_data, &auxiliaries_data, &conjugates_data, &conjugations_data).await;
// }
//
// fn extract_languages(languages_data: Vec<JsonData>) -> Vec<&'static str> {
//     let mut languages: Vec<&str> = Vec::new();
//     for language_data in languages_data {
//         if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
//             languages.push(language);
//         }
//     }
//
//     return languages;
// }
//
//
//
// fn form_verb_url(languages) {
//     for language in languages {
//         //
//     }
// }
//
//
//
// fn form_conjugation_url(language: &str, verb: &str) -> String {
//     return String::from("https::/conjugator.reverso.net/conjugation-") + language + "-verb-" + verb + ".html";
// }
//
//
//
//
//
//
//
//
// async fn save_data_to_postgres(bases_data: &Vec<JsonData>, tenses_data: &Vec<JsonData>, subjects_data: &Vec<JsonData>, auxiliaries_data: &Vec<JsonData>, conjugates_data: &Vec<JsonData>, conjugations_data: &Vec<JsonData>) {
//     let pool = create_pool_connection().await;
//
//     for bases_data in groups_data {
//         println!("{:?}, {:?}", group_data, group_data.pk);
//         if let Field::GroupField(GroupField{language, group}) = &group_data.fields {
//
//             //if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
//                 .bind(group_data.pk)
//                 .bind(language)
//                 .bind(group)
//                 .execute(&pool)
//                 .await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
//                         .bind(language)
//                         .bind(group)
//                         .bind(group_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//
//         } else {
//             panic!("non-group in group field");
//         };
//     }
//
//
//     for ending_data in endings_data {
//         if let Field::EndingField(EndingField { group, ending }) = &ending_data.fields {
//             // if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
//                 .bind(ending_data.pk)
//                 .bind(group)
//                 .bind(ending)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_ending SET group=($1), ending=($2), WHERE id=($3)")
//                         .bind(group)
//                         .bind(ending)
//                         .bind(ending_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 }
//             };
//         } else {
//             panic!("non-ending in ending field");
//         };
//     }
//     
//
//     for model_data in models_data {
//         println!("{:?} {:?}", model_data, model_data.pk);
//         if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
//             let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
//                 .bind(model_data.pk)
//                 .bind(ending)
//                 .bind(model)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
//                         .bind(ending)
//                         .bind(model)
//                         .bind(model_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//         } else {
//             panic!("non-model in model field");
//         };
//     }
//
//
//
//
//     for group_data in groups_data {
//         println!("{:?}, {:?}", group_data, group_data.pk);
//         if let Field::GroupField(GroupField{language, group}) = &group_data.fields {
//
//             //if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
//                 .bind(group_data.pk)
//                 .bind(language)
//                 .bind(group)
//                 .execute(&pool)
//                 .await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
//                         .bind(language)
//                         .bind(group)
//                         .bind(group_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//
//         } else {
//             panic!("non-group in group field");
//         };
//     }
//
//
//     for conjugates_data in endings_data {
//         if let Field::EndingField(EndingField { group, ending }) = &ending_data.fields {
//             // if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
//                 .bind(ending_data.pk)
//                 .bind(group)
//                 .bind(ending)
//                         .bind(group)
//                         .bind(ending)
//                         .bind(ending_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 }
//             };
//         } else {
//             panic!("non-ending in ending field");
//         };
//     }
//     
//
//     for conjugations_data in models_data {
//         println!("{:?} {:?}", model_data, model_data.pk);
//         if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
//             let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
//                 .bind(model_data.pk)
//                 .bind(ending)
//                 .bind(model)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
//                         .bind(ending)
//                         .bind(model)
//                         .bind(model_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//         } else {
//             panic!("non-model in model field");
//         };
//     }
// }
