use crate::{
    data_types::{
        page_info::{PageInfo, Tense, Phrase},
        json_data::{JsonData, create_json_data_vec_from_vec_vec_string},
        field::{Field, FieldOptions},
        field_options::{
            LanguageField,
            BaseField,
            TenseField,
            SubjectField,
            AuxiliaryField,
            ConjugateField,
            ConjugationField,
        },
    },
    helper_functions::{
        // postgres_functions::save_data_to_postgres,
        file_operations::create_file,
        read_functions::{read_file_to_string, read_file_to_string_none},
        save_functions::{
            save_env,
            save_json_data_vec_to_file,
            save_map_vec_to_file,
            save_string_vec_vec_to_file, save_data_to_file,
        },
    }
};

use std::{
    env,
    collections::BTreeMap,
    time::Duration,
    thread,
    fs::read_to_string,
};

use itertools::Itertools;
use scraper::Node;


pub async fn run_conjugations_modules() {
    // Read language data from file
    let language_content: String = read_file_to_string("temp/json/languages/languages.json");
    let (_language_data_vec, language_vec) = read_language_data_from_json_data(language_content.as_str());
    // println!("language_data_vec: {:#?}", _language_data_vec);

    // Read group data from file
    let group_content: String = read_file_to_string("temp/json/models/groups.json");
    let _group_data_vec: Vec<JsonData> = serde_json::from_str(group_content.as_str()).unwrap();
    // println!("group_data_vec: {:#?}", _group_data_vec);

    // Read ending data from file
    let ending_content: String = read_file_to_string("temp/json/models/endings.json");
    let _ending_data_vec: Vec<JsonData> = serde_json::from_str(ending_content.as_str()).unwrap();
    // println!("ending_data_vec: {:#?}", _ending_data_vec);

    // Read model data from file
    let model_content: String = read_file_to_string("temp/json/models/models.json");
    let _model_data_vec: Vec<JsonData> = serde_json::from_str(model_content.as_str()).unwrap();
    // println!("model_data_vec: {:#?}", _model_data_vec);

    // Get regular exponential back off & error 429 backoff
    let mut backoff: u64 = env::var("BACKOFF").unwrap().parse::<u64>().unwrap();
    let mut error_429_backoff: u64 = env::var("ERROR_429_BACKOFF").unwrap().parse::<u64>().unwrap();

    // Fetch verb urls vector
    let (verb_url_vec_vec, backoff_res, error_429_backoff_res) = fetch_verb_url_vec_vec(language_vec.clone(), backoff, error_429_backoff).await;

    backoff = backoff_res;
    error_429_backoff = error_429_backoff_res;
    // let mut backoff_duration = time::Duration::from_secs(backoff);
    // let mut error_429_backoff_duration = time::Duration::from_secs(error_429_backoff);

    // Generate verb page information vector
    let verb_page_info_vec_vec: Vec<Vec<PageInfo>> = generate_verb_page_info_vec_vec(language_vec.clone(), verb_url_vec_vec, backoff, error_429_backoff).await;
    // println!("verb_page_info_vec: {:#?}", verb_page_info_vec);




    panic!("\n\n\npause here boss");
    // Read necessary pk map vec
    let language_pk_map_vec: Vec<BTreeMap<String, i64>> = serde_json::from_str(&language_content).unwrap();
    let model_content: String = read_file_to_string("temp/json/models/btreemaps/model_language.json");
    let model_language_id_map_vec: Vec<BTreeMap<String, i64>> = serde_json::from_str(&model_content).unwrap();


    // Fetch data vectors and then create and save json data vectors
    let base_data_vec_vec: Vec<Vec<String>> = extract_base_data_vec_vec(&verb_page_info_vec_vec, &language_pk_map_vec);
    let base_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(base_data_vec_vec, FieldOptions::BaseField);
    save_json_data_vec_to_file(&base_json_data_vec, "temp/json/conjugations/bases.json");
    let base_pk_map_vec: Vec<BTreeMap<String, i64>> = get_base_pk_map_vec(base_json_data_vec.clone());
    save_map_vec_to_file(&base_pk_map_vec, "temp/json/conjugations/btreemaps/bases.json");

    let tense_data_vec_vec: Vec<Vec<String>> = extract_tense_data_vec_vec(&verb_page_info_vec_vec, &language_pk_map_vec);
    let tense_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(tense_data_vec_vec, FieldOptions::TenseField);
    save_json_data_vec_to_file(&tense_json_data_vec, "temp/json/conjugations/tenses.json");
    let tense_pk_map_vec: Vec<BTreeMap<String, i64>> = get_tense_pk_map_vec(tense_json_data_vec);
    save_map_vec_to_file(&tense_pk_map_vec, "temp/json/conjugations/btreemaps/tenses.json");

    let subject_data_vec_vec: Vec<Vec<String>> = extract_subject_data_vec_vec(&verb_page_info_vec_vec, &language_pk_map_vec);
    let subject_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(subject_data_vec_vec, FieldOptions::SubjectField);
    save_json_data_vec_to_file(&subject_json_data_vec, "temp/json/conjugations/subjects.json");
    let subject_pk_map_vec: Vec<BTreeMap<String, i64>> = get_subject_pk_map_vec(subject_json_data_vec);
    save_map_vec_to_file(&subject_pk_map_vec, "temp/json/conjugations/btreemaps/subjects.json");

    let auxiliary_data_vec_vec: Vec<Vec<String>> = extract_auxiliary_data_vec_vec(&verb_page_info_vec_vec, &language_pk_map_vec);
    let auxiliary_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(auxiliary_data_vec_vec, FieldOptions::AuxiliaryField);
    save_json_data_vec_to_file(&auxiliary_json_data_vec, "temp/json/conjugations/auxiliaries.json");
    let auxiliary_pk_map_vec: Vec<BTreeMap<String, i64>> = get_auxiliary_pk_map_vec(auxiliary_json_data_vec);
    save_map_vec_to_file(&auxiliary_pk_map_vec, "temp/json/conjugations/btreemaps/auxiliaries.json");

    let conjugate_data_vec_vec: Vec<Vec<String>> = extract_conjugate_data_vec_vec(&verb_page_info_vec_vec, &base_pk_map_vec, &model_language_id_map_vec);
    let conjugate_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(conjugate_data_vec_vec, FieldOptions::ConjugateField);
    save_json_data_vec_to_file(&conjugate_json_data_vec, "temp/json/conjugations/conjugates.json");
    let conjugate_pk_map_vec: Vec<BTreeMap<String, i64>> = get_conjugate_pk_map_vec(conjugate_json_data_vec.clone(), base_json_data_vec.clone());
    save_map_vec_to_file(&conjugate_pk_map_vec, "temp/json/conjugations/btreemaps/conjugate.json");

    let conjugation_data_vec_vec: Vec<Vec<String>> = extract_conjugation_data_vec_vec(
        &verb_page_info_vec_vec, &tense_pk_map_vec, &subject_pk_map_vec, &auxiliary_pk_map_vec, &conjugate_pk_map_vec);
    let conjugation_json_data_vec: Vec<JsonData> = create_json_data_vec_from_vec_vec_string(conjugation_data_vec_vec, FieldOptions::ConjugationField);
    save_json_data_vec_to_file(&conjugation_json_data_vec, "temp/json/conjugations/conjugations.json");
    let conjugation_pk_map_vec: Vec<BTreeMap<String, i64>> = get_conjugation_pk_map_vec(conjugation_json_data_vec, conjugate_json_data_vec, base_json_data_vec);
    save_map_vec_to_file(&conjugation_pk_map_vec, "temp/json/conjugations/btreemaps/conjugation.json");
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


async fn fetch_verb_url_vec_vec(language_vec: Vec<String>, backoff: u64, error_429_backoff: u64) -> (Vec<Vec<String>>, u64, u64) {
// try to read urls_vec_vec otherwise scrape from reverso
    let verb_url_vec_vec_file_path: &str = "temp/json/conjugations/verb_urls.json";
    let verb_url_vec_content: String = read_file_to_string_none(verb_url_vec_vec_file_path);
    let verb_url_vec_vec_file_result: Result<Vec<Vec<String>>, serde_json::Error>
            = serde_json::from_str(verb_url_vec_content.as_str());


    let verb_url_vec_vec: Vec<Vec<String>> = match verb_url_vec_vec_file_result {
        Ok(verb_url_vec_vec) => verb_url_vec_vec,

        Err(_) => {
            let url_listing_vec_vec = generate_url_listing_vec_vec(&language_vec);
            let (verb_vec_vec, backoff, error_429_backoff) = scrape_url_listing_vec_vec(url_listing_vec_vec, backoff, error_429_backoff).await;

            let verb_url_vec_vec: Vec<Vec<String>> = generate_verb_url_vec_vec(verb_vec_vec, language_vec);
            save_string_vec_vec_to_file(&verb_url_vec_vec, "temp/json/conjugations/verb_urls.json");

            return (verb_url_vec_vec, backoff, error_429_backoff);
        }
    };

    // println!("verb_url_vec_vec: {:?}", verb_url_vec_vec);
    return (verb_url_vec_vec, backoff, error_429_backoff);
}


fn generate_url_listing_vec_vec(language_vec: &Vec<String>) -> Vec<Vec<String>> {
    let mut url_listing_vec_vec: Vec<Vec<String>> = Vec::new();

    let url_val_array: [i64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    for language in language_vec {
        let url_listing_vec: Vec<String> = url_val_array.map(|val|
            String::from("https://conjugator.reverso.net/index-")
            + language.as_str() + "-" + &((250*val) + 1).to_string()
            + "-" + &(250*(val + 1)).to_string() + ".html")
            .to_vec();

        url_listing_vec_vec.push(url_listing_vec);
    }

    return url_listing_vec_vec;
}

async fn scrape_url_listing_vec_vec(url_listing_vec_vec: Vec<Vec<String>>, mut backoff: u64, mut error_429_backoff: u64) -> (Vec<Vec<String>>, u64, u64) {
    let mut verb_vec_vec: Vec<Vec<String>> = url_listing_vec_vec.clone().into_iter().map(|_| Vec::new()).collect::<Vec<Vec<String>>>();
        
    for (index, url_listing_vec) in url_listing_vec_vec.into_iter().enumerate() {
        for url_listing in url_listing_vec {
            let mut valid_response_bool: bool = false;
            let mut response_loop_count: usize = 0;
            let mut response: String = String::new();

            while valid_response_bool == false {
                let request = reqwest::get(url_listing.clone()).await.unwrap();

                match request.status() {
                    reqwest::StatusCode::OK => valid_response_bool = true,
                    // reqwest::StatusCode::TOO_MANY_REQUESTS => panic!("Too many requests"),
                    // other => panic!("{:?}", other),
                    _ => {}
                };

                response = request.text().await.unwrap();

                if valid_response_bool == false {

                    if response_loop_count == 0 {
                        backoff = ((backoff + 1) as f64 * 1.2).round() as u64;
                        env::set_var("BACKOFF", backoff.to_string());
                        save_env("BACKOFF", &backoff.to_string()).unwrap();
                        // save new backoff to env
                    } else {
                        error_429_backoff = ((error_429_backoff + 1) as f64 * 1.2).round() as u64;
                        env::set_var("ERROR_429_BACKOFF", error_429_backoff.to_string());
                        save_env("ERROR_429_BACKOFF", &error_429_backoff.to_string()).unwrap();
                        // save new error_429_backoff to env
                    }

                    let error_429_backoff_duration: Duration = Duration::from_secs(error_429_backoff);
                    thread::sleep(error_429_backoff_duration);
                    response_loop_count += 1;
                }
            }

            let mut content: String = String::new();
            content.push_str(response.as_str());
            let document = scraper::Html::parse_document(&content);
            
            let section_container = scraper::Selector::parse("div.index-content>ol").unwrap();
            let section = document.select(&section_container).next().unwrap();

            // map to get the vec of verbs
            let li_selector = scraper::Selector::parse("li").unwrap();
            let mut verb_vec: Vec<String> = section.select(&li_selector).map(|li| li.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
            validate_verb_vec(&mut verb_vec);

            verb_vec_vec[index].append(&mut verb_vec);
            println!("verb_vec_vec[{}]: {:?}", index, verb_vec_vec[index]);

            // wait the backoff duration
            let backoff_duration: Duration = Duration::from_secs(backoff);
            thread::sleep(backoff_duration);
        }
    }

    return (verb_vec_vec, backoff, error_429_backoff);
}


fn validate_verb_vec(_verb_vec: &Vec<String>) {
}


fn generate_verb_url_vec_vec(verb_vec_vec: Vec<Vec<String>>, language_vec: Vec<String>) -> Vec<Vec<String>> {
    let verb_url_vec_vec: Vec<Vec<String>> = verb_vec_vec.into_iter().enumerate()
        .map(|(index, verb_vec)|
            verb_vec.into_iter().map(|verb| String::from("https://conjugator.reverso.net/conjugation-")
                + language_vec[index].as_str() + "-verb-" + verb.as_str() + ".html")
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    return verb_url_vec_vec;
}


// take a page such as https://conjugator.reverso.net/conjugation-french-verb-faire.html
// and turn get all the data out of it and put into the PageInfo struct
// inner vec for each verb in a language
// outer vec for each language
async fn generate_verb_page_info_vec_vec(language_vec: Vec<String>, verb_url_vec_vec: Vec<Vec<String>>, mut backoff: u64, mut error_429_backoff: u64) -> Vec<Vec<PageInfo>> {
    let mut page_info_vec_vec: Vec<Vec<PageInfo>> = Vec::new();

    for (index, verb_url_vec) in verb_url_vec_vec.into_iter().enumerate() {
        let mut page_info_vec: Vec<PageInfo> = Vec::new();
        let mut count: u64 = 0;

        for verb_url in verb_url_vec {
            count += 1;

            // check if verb has already been scraped
            let verb: String = verb_url[0..verb_url.find(".html").unwrap()]
                .split('-').last().unwrap().to_string();
            let page_info_vec_file_path: String = String::from("temp/json/conjugations/") + language_vec[index].as_str() + "_page_info.json";
            let scraped_verb_vec_file_path: String = String::from("temp/json/conjugations/") + language_vec[index].as_str() + "_scraped_verb_vec.json";
            
            let scraped_verb_content: String = match read_to_string(scraped_verb_vec_file_path.as_str()) { 
                Ok(content) => content,
                Err(_) => {
                    create_file(&scraped_verb_vec_file_path).unwrap();
                    String::new()
                },
            };

            let mut scraped_verb_vec: Vec<String> = match serde_json::from_str(&scraped_verb_content) {
                Ok(content) => content,
                Err(_) => Vec::new(),
            };

            if scraped_verb_vec.contains(&verb) {
                continue;
            }

            let mut valid_response_bool: bool = false;
            let mut response_loop_count: usize = 0;
            let mut response: String = String::new();

            let backoff_duration: Duration = Duration::from_secs(backoff);
            thread::sleep(backoff_duration);

            while valid_response_bool == false {
                let request = reqwest::get(verb_url.clone()).await.unwrap();

                match request.status() {
                    reqwest::StatusCode::OK => valid_response_bool = true,
                    // reqwest::StatusCode::TOO_MANY_REQUESTS => panic!("Too many requests"),
                    // other => panic!("{:?}", other),
                    _ => {},
                };

                response = request.text().await.unwrap();

                if valid_response_bool == false {

                    if response_loop_count == 0 {
                        backoff = ((backoff + 1) as f64 * 1.2).round() as u64;
                        env::set_var("BACKOFF", backoff.to_string());
                        save_env("BACKOFF", &backoff.to_string()).unwrap();
                        // save new backoff to env
                    } else {
                        error_429_backoff = ((error_429_backoff + 1) as f64 * 1.2).round() as u64;
                        env::set_var("ERROR_429_BACKOFF", error_429_backoff.to_string());
                        save_env("ERROR_429_BACKOFF", &error_429_backoff.to_string()).unwrap();
                        // save new error_429_backoff to env
                    }

                    let error_429_backoff_duration: Duration = Duration::from_secs(error_429_backoff);
                    thread::sleep(error_429_backoff_duration);
                    response_loop_count += 1;
                }
            }

            // do earlier validation
            let mut content: String = String::new();
            content.push_str(response.as_str());
            let document = scraper::Html::parse_document(&content);

            let mut page_info: PageInfo = PageInfo::new();

            // metadata
            // let metadata_section_selector = scraper::Selector::parse("").unwrap();
            let model_selector = scraper::Selector::parse("span#ch_lblModel>a").unwrap();
            let base_selector = scraper::Selector::parse("a#ch_lblVerb").unwrap();
            let auxiliary_selector = scraper::Selector::parse("span#ch_lblAuxiliary>a").unwrap();
            let forms_selector = scraper::Selector::parse("span#ch_lblAutreForm>a").unwrap();
            let similar_verbs_selector = scraper::Selector::parse("div.word-wrap-descr>a").unwrap();
            let other_verbs_selector = scraper::Selector::parse("div.verb-others-list>a").unwrap();

            // let metadata_section = document.select(&metadata_section_selector).next().unwrap();
            page_info.metadata.language = language_vec[index].clone();
            page_info.metadata.rank = count.to_string();
            page_info.metadata.model = document.select(&model_selector).next().unwrap().text().collect::<String>().trim().to_string();
            page_info.metadata.base = document.select(&base_selector).next().unwrap().text().collect::<String>().trim().to_string();
            page_info.metadata.auxiliary = document.select(&auxiliary_selector).map(|auxiliary| auxiliary.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
            page_info.metadata.forms = document.select(&forms_selector).map(|form| form.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
            page_info.metadata.similar_verbs = document.select(&similar_verbs_selector).map(|similar| similar.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
            page_info.metadata.other_verbs = document.select(&other_verbs_selector).map(|other| other.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();

            // data
            let subjects_selector = scraper::Selector::parse("i.graytxt").unwrap();
            let auxiliaries_selector = scraper::Selector::parse("i.auxgraytxt").unwrap();
            let conjugates_selector = scraper::Selector::parse("i.verbtxt").unwrap();
            let particles_selector = scraper::Selector::parse("i.particletxt").unwrap();

            page_info.subjects = document.select(&subjects_selector).map(|subject| subject.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>();
            page_info.auxiliaries = document.select(&auxiliaries_selector).map(|auxiliary| auxiliary.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>();
            page_info.conjugates = document.select(&conjugates_selector).map(|conjugate| conjugate.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>();
            page_info.particles = document.select(&particles_selector).map(|particle| particle.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>();

            let main_section_selector = scraper::Selector::parse("div.word-wrap").unwrap();
            let tense_selector = scraper::Selector::parse("div[mobile-title]").unwrap();
            let major_tense_selector = scraper::Selector::parse("div.word-wrap-title>h4").unwrap();

            let main_section = document.select(&main_section_selector).next().unwrap();
            // let minor_tense = main_section.select(&minor_tense_selector).map(|minor_tense| minor_tense.text().collect::<String>()).collect::<Vec<String>>();

            // Get tenses
            let tense_vec: Vec<String> = main_section.select(&tense_selector).map(|tense| tense.value().attr("mobile-title").unwrap().trim().to_string())
                .collect::<Vec<String>>();
            // println!("tense_vec: {:?}", tense_vec);

            // Get major tense parts
            let major_tense_vec = main_section.select(&major_tense_selector).map(|major_tense| major_tense.text().collect::<String>().trim().to_string())
                .collect::<Vec<String>>().into_iter().unique().collect::<Vec<String>>();
            // println!("major_tense_vec: {:?}", major_tense_vec);

            // Derive minor tenses
            let mut minor_tense_vec: Vec<String> = Vec::new();
            let mut tense_struct_vec: Vec<Tense> = Vec::new();

            for tense in &tense_vec {
                let minor_tense;
                for major_tense in &major_tense_vec {
                    if tense.contains(major_tense) {
                        minor_tense = tense.replace(major_tense, "").trim().to_string();
                        if minor_tense.len() > 0 { minor_tense_vec.push(minor_tense.clone()) };

                        tense_struct_vec.push(Tense {
                            major: major_tense.to_owned(),
                            minor: minor_tense,
                        });

                        break;
                    }

                }
            }

            page_info.major_tenses = major_tense_vec.into_iter().unique().collect();
            page_info.minor_tenses = minor_tense_vec.into_iter().unique().collect();
            page_info.tenses = tense_struct_vec;



            let mini_section = scraper::Selector::parse("div.wrap-three-col").unwrap();
            let phrase_selector = scraper::Selector::parse("li").unwrap();
            let conjugates_fused_selector = scraper::Selector::parse("i[h]").unwrap();
            let i_selector = scraper::Selector::parse("i").unwrap();

            for mini_section in main_section.select(&mini_section) {
                let mut phrase_vec: Vec<Phrase> = Vec::new();
                for phrase_section in mini_section.select(&phrase_selector) {
                    let mut phrase: Vec<String> = phrase_section.select(&i_selector)
                        .map(|phrase| {
                            match phrase.first_child().unwrap().has_children() {
                                true => phrase.text().collect::<String>().trim().to_string(),
                                false => {
                                    // let mut res = String::new();
                                    // match phrase.parent().unwrap().value().clone() {
                                    //     Node::Element(elem) => {
                                    //         let near = elem.name();
                                    //         if near == "li" {
                                    //             res = phrase.text().collect::<String>().trim().to_string()
                                    //         }
                                    //         res
                                    //         // println!("elem: {:?}", near);
                                    //     },
                                    //     _ => String::new(),
                                    // }
                                    let parent_value = phrase.parent().unwrap().value();
                                    if let Node::Element(element) = parent_value {
                                        if element.name() == "li" {
                                            phrase.text().collect::<String>().trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() }
                                },
                            }
                        })
                        .collect::<Vec<String>>();
                    phrase.retain(|s| s.len() > 0);
                    phrase = match phrase.len() {
                        0 => Vec::new(),
                        _ => phrase,
                    };
                    // println!("phrase: {:?}", phrase);

                    let mut other: Vec<String> = phrase.clone();
                    other.sort();

                    let mut subjects: Vec<String> = phrase_section.select(&subjects_selector).map(|subject| subject.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
                    subjects = match subjects.len() {
                        0 => Vec::new(),
                        _ => {
                            for subject in subjects.iter() {
                                other.remove(other.binary_search(subject).unwrap());
                            }
                            subjects
                        },
                    };

                    let mut auxiliaries: Vec<String> = phrase_section.select(&auxiliaries_selector).map(|auxiliary| auxiliary.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
                    auxiliaries = match auxiliaries.len() {
                        0 => Vec::new(),
                        _ => {
                            for auxiliary in auxiliaries.iter() {
                                other.remove(other.binary_search(auxiliary).unwrap());
                            }
                            auxiliaries
                        },
                    };

                    let conjugates_fused: Vec<String> = phrase_section.select(&conjugates_fused_selector).map(|test| test.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
                    let mut conjugates: Vec<String> = phrase_section.select(&conjugates_selector).map(|conjugate| conjugate.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
                    conjugates = match conjugates_fused.len() {
                        0 => match conjugates.len() {
                            0 => Vec::new(),
                            _ => {
                                for conjugate in conjugates.iter() {
                                    other.remove(other.binary_search(conjugate).unwrap());
                                }
                                conjugates
                            },
                        },
                        _ => {
                            for conjugate in conjugates_fused.iter() {
                                other.remove(other.binary_search(conjugate).unwrap());
                            }
                            conjugates_fused.to_owned()
                        },
                    };

                    let mut particles: Vec<String> = phrase_section.select(&particles_selector).map(|particle| particle.text().collect::<String>().trim().to_string()).collect::<Vec<String>>();
                    particles = match particles.len() {
                        0 => Vec::new(),
                        _ => {
                            for particle in particles.iter() {
                                other.remove(other.binary_search(particle).unwrap());
                            }
                            particles
                        },
                    };
 
                    let phrase: Phrase = Phrase {phrase, subjects, auxiliaries, conjugates, particles, other};

                    phrase_vec.push(phrase);
                }

                page_info.phrases.push(phrase_vec);
            }

            page_info_vec.push(page_info);
            scraped_verb_vec.push(verb);
            
            save_data_to_file(&page_info_vec, &page_info_vec_file_path);
            save_data_to_file(&scraped_verb_vec, &scraped_verb_vec_file_path);

            println!("page_info_vec: {:#?}", page_info_vec);
            // panic!("\n\n\npause here boss");
        }

        page_info_vec_vec.push(page_info_vec);
    }

    return page_info_vec_vec;
}


fn extract_base_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, language_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let mut base_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, verb_page_info_vec) in verb_page_info_vec_vec.into_iter().enumerate() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let rank: String = verb_page_info.metadata.rank.clone();
            let language: String = language_pk_map_vec[index].get(&verb_page_info.metadata.language.clone()).unwrap().to_string();
            let base: String = verb_page_info.metadata.base.clone();

            let base_data_vec = Vec::from([rank, language, base]);

            if base_data_vec_vec.contains(&base_data_vec) == false {
                base_data_vec_vec.push(base_data_vec);
            }
        }
    }
    
    return base_data_vec_vec;
}


fn get_base_pk_map_vec(base_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut base_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for base_data in base_json_data_vec {
        let mut base_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::BaseField(BaseField { rank:_, base, language }) = &base_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            base_pk_map.insert(base.to_owned(), base_data.pk);

            if language_id >= base_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                base_pk_map_vec.push(base_pk_map);
            } else {
                base_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut base_pk_map);
            }
        }
    }

    return base_pk_map_vec;
}


fn extract_tense_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, language_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let mut tense_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, verb_page_info_vec) in verb_page_info_vec_vec.into_iter().enumerate() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language: String = language_pk_map_vec[index].get(&verb_page_info.metadata.language.clone()).unwrap().to_string();
            let tense_data_vec: Vec<Tense> = verb_page_info.tenses.clone();

            for tense_data in tense_data_vec {
                let tense: Vec<String> = Vec::from([language.clone(), tense_data.major, tense_data.minor]);

                if tense_data_vec_vec.contains(&tense) {
                    tense_data_vec_vec.push(tense);
                }
            }
        }
    }

    return tense_data_vec_vec;
}


fn get_tense_pk_map_vec(base_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut tense_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for tense_data in base_json_data_vec {
        let mut tense_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::TenseField(TenseField { rank:_, language, tense }) = &tense_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            tense_pk_map.insert(tense.major.clone() + "|" + tense.minor.as_str(), tense_data.pk);

            if language_id >= tense_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                tense_pk_map_vec.push(tense_pk_map);
            } else {
                tense_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut tense_pk_map);
            }
        }
    }

    return tense_pk_map_vec;
}


fn extract_subject_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, language_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let mut subject_data_vec_vec: Vec<Vec<String>> = Vec::new();
    
    for (index, verb_page_info_vec) in verb_page_info_vec_vec.into_iter().enumerate() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language: String = language_pk_map_vec[index].get(&verb_page_info.metadata.language.clone()).unwrap().to_string();
            let subjects: Vec<String> = verb_page_info.subjects.clone();

            for subject in subjects {
                let subject_data_vec: Vec<String> = Vec::from([language.clone(), subject]);
                if subject_data_vec_vec.contains(&subject_data_vec) == false {
                    subject_data_vec_vec.push(subject_data_vec);
                }
                
            }
        }
    }

    return subject_data_vec_vec;
}


fn get_subject_pk_map_vec(subject_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut subject_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for subject_data in subject_json_data_vec {
        let mut subject_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::SubjectField(SubjectField { rank:_, subject, language }) = &subject_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            subject_pk_map.insert(subject.to_owned(), subject_data.pk);

            if language_id >= subject_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                subject_pk_map_vec.push(subject_pk_map);
            } else {
                subject_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut subject_pk_map);
            }
        }
    }

    return subject_pk_map_vec;
}


fn extract_auxiliary_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, language_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let mut auxiliary_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, verb_page_info_vec) in verb_page_info_vec_vec.into_iter().enumerate() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let language: String = language_pk_map_vec[index].get(&verb_page_info.metadata.language.clone()).unwrap().to_string();
            let auxiliaries: Vec<String> = verb_page_info.auxiliaries.clone(); 

            for auxiliary in auxiliaries {
                let auxiliary_data_vec: Vec<String> = Vec::from([language.clone(), auxiliary]);
                if auxiliary_data_vec_vec.contains(&auxiliary_data_vec) {
                    auxiliary_data_vec_vec.push(auxiliary_data_vec);
                }
            }
        }
    }

    return auxiliary_data_vec_vec
}


fn get_auxiliary_pk_map_vec(auxiliary_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut auxiliary_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for auxiliary_data in auxiliary_json_data_vec {
        let mut auxiliary_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::AuxiliaryField(AuxiliaryField { auxiliary, language }) = &auxiliary_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            auxiliary_pk_map.insert(auxiliary.to_owned(), auxiliary_data.pk);

            if language_id >= auxiliary_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                auxiliary_pk_map_vec.push(auxiliary_pk_map);
            } else {
                auxiliary_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut auxiliary_pk_map);
            }
        }
    }

    return auxiliary_pk_map_vec;
}


fn extract_conjugate_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, base_pk_map_vec: &Vec<BTreeMap<String, i64>>, model_language_id_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let mut conjugate_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, verb_page_info_vec) in verb_page_info_vec_vec.into_iter().enumerate() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            let base: String = base_pk_map_vec[index].get(&verb_page_info.metadata.base.clone()).unwrap().to_string();
            let model: String = model_language_id_map_vec[index].get(&verb_page_info.metadata.model.clone()).unwrap().to_string();
            let conjugates: Vec<String> = verb_page_info.conjugates.clone();
            for conjugate in conjugates {
                let conjugate_data_vec: Vec<String> = Vec::from([base.clone(), model.clone(), conjugate]);
                if conjugate_data_vec_vec.contains(&conjugate_data_vec) {
                    conjugate_data_vec_vec.push(conjugate_data_vec);
                }
            }
        }
    }

    return conjugate_data_vec_vec;
}


fn get_conjugate_pk_map_vec(conjugate_json_data_vec: Vec<JsonData>, base_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut conjugate_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for conjugate_data in conjugate_json_data_vec {
        let mut conjugate_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::ConjugateField(ConjugateField { rank:_, base, model:_, conjugate }) = &conjugate_data.fields {
            let base_pk: i64 = base.parse::<i64>().unwrap() - 1;
            conjugate_pk_map.insert(conjugate.to_owned(), conjugate_data.pk);
            
            let mut language_id: i64 = 0;
            if let Field::BaseField(BaseField { rank:_, language, base:_ }) = base_json_data_vec[base_pk.to_string().parse::<usize>().unwrap()].clone().fields {
                language_id = language.parse::<i64>().unwrap();
            }
            if language_id >= conjugate_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                conjugate_pk_map_vec.push(conjugate_pk_map);
            } else {
                conjugate_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut conjugate_pk_map);
            }
        }
    }

    return conjugate_pk_map_vec;
}


fn extract_conjugation_data_vec_vec(verb_page_info_vec_vec: &Vec<Vec<PageInfo>>, tense_pk_map_vec: &Vec<BTreeMap<String, i64>>,
    subject_pk_map_vec: &Vec<BTreeMap<String, i64>>, auxiliary_pk_map_vec: &Vec<BTreeMap<String, i64>>,
    conjugate_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    
    let mut conjugation_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for verb_page_info_vec in verb_page_info_vec_vec.into_iter() {
        for verb_page_info in verb_page_info_vec.into_iter() {
            // Need to use a different rank
            let phrases: Vec<Vec<Phrase>> = verb_page_info.phrases.clone();

            for (index, phrase_vec) in phrases.into_iter().enumerate() {
                for phrase in phrase_vec {
                    // make tense_pk_map_vec a Vec<BTreeMap<Tense, i64>>?
                    let rank: String = verb_page_info.metadata.rank.clone();
                    let tense = tense_pk_map_vec[index].get(&(verb_page_info.tenses[index].clone().major+ " " + verb_page_info.tenses[index].clone().minor.as_str())).unwrap().to_string();
                    let subject = subject_pk_map_vec[index].get(&phrase.subjects[0]).unwrap().to_string();
                    let auxiliary = auxiliary_pk_map_vec[index].get(&phrase.auxiliaries[0]).unwrap().to_string();
                    let conjugate = conjugate_pk_map_vec[index].get(&phrase.conjugates[0]).unwrap().to_string();

                    let conjugation_data_vec: Vec<String> = Vec::from([rank, tense, subject, auxiliary, conjugate]);
                    conjugation_data_vec_vec.push(conjugation_data_vec);
                }
            }
        }
    }

    return conjugation_data_vec_vec;
}


fn get_conjugation_pk_map_vec(conjugation_json_data_vec: Vec<JsonData>, conjugate_json_data_vec: Vec<JsonData>, base_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>>  {
    let mut conjugation_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();
    for conjugation_data in conjugation_json_data_vec {
        let mut conjugation_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::ConjugationField(ConjugationField { rank:_, tense:_, subject:_, auxiliary:_, conjugate }) = &conjugation_data.fields {
            let conjugate_pk: i64 = conjugate.parse::<i64>().unwrap() - 1;

            let mut base_pk: i64 = 0;
            if let Field::ConjugateField(ConjugateField { rank:_, conjugate:_, base, model:_ }) = conjugate_json_data_vec[conjugate_pk.to_string().parse::<usize>().unwrap()].clone().fields {
                base_pk = base.parse::<i64>().unwrap() - 1;
            } 

            let mut language_id: i64 = 0;
            if let Field::BaseField(BaseField { rank:_, language, base:_ }) = base_json_data_vec[base_pk.to_string().parse::<usize>().unwrap()].clone().fields {
                language_id = language.parse::<i64>().unwrap();
            }

            
            conjugation_pk_map.insert(conjugate.to_owned(), conjugation_data.pk);

            if language_id >= conjugation_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                conjugation_pk_map_vec.push(conjugation_pk_map);
            } else {
                conjugation_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut conjugation_pk_map);
            }
        }
    }

    return conjugation_pk_map_vec;
}

