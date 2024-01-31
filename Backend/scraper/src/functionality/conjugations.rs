// Todo
use crate::data_types::{
    page_info::{
        PageInfo,
        Tense,
        Phrase
    },
    json_data::{
        JsonData,
        create_json_data_vec_from_vec_vec_string
    },
    field::{
        Field,
        FieldOptions,
    },
    field_options::{
        LanguageField,
        // GroupField,
        // EndingField,
        // ModelField,
        BaseField,
        TenseField,
        SubjectField,
        AuxiliaryField,
        ConjugateField,
        ConjugationField,
    }
};

use crate::helper_functions::{
    // postgres_functions::save_data_to_postgres,
    read_functions::read_file_to_string,
    save_functions::{
        save_json_data_vec_to_file,
        save_map_vec_to_file,
        save_string_vec_vec_to_file,
    },

};

use std::{
    env,
    collections::BTreeMap,
    // result,
    time::{
        self,
        Duration,
    },
    thread,
};



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
    let backoff: u64 = env::var("BACKOFF").unwrap().parse::<u64>().unwrap();
    let backoff_duration: Duration = time::Duration::from_secs(backoff);
    let error_429_backoff: u64 = env::var("ERROR_429_BACKOFF").unwrap().parse::<u64>().unwrap();
    let error_429_backoff_duration: Duration = time::Duration::from_secs(error_429_backoff);
    println!("backoff: {}, error 429 backoff: {}", backoff, error_429_backoff);

    // Fetch verb urls vector
    let verb_url_vec_vec: Vec<Vec<String>> = fetch_verb_url_vec_vec(language_vec, backoff_duration, error_429_backoff_duration);

    // Generate verb page information vector
    let verb_page_info_vec_vec: Vec<Vec<PageInfo>> = generate_verb_page_info_vec_vec(verb_url_vec_vec, backoff_duration, error_429_backoff_duration);
    // println!("verb_page_info_vec: {:#?}", verb_page_info_vec);




    // Read necessary pk map vec
    let language_content: String = read_file_to_string("temp/json/languages/languages.json");
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


fn fetch_verb_url_vec_vec(language_vec: Vec<String>, backoff_duration: Duration, error_429_backoff_duration: Duration) -> Vec<Vec<String>> {
// try to read urls_vec_vec otherwise scrape from reverso
    let verb_url_vec_vec_file_path: &str = "temp/json/conjugations/verb_urls.json";
    let verb_url_vec_content: String = read_file_to_string(verb_url_vec_vec_file_path);
    let verb_url_vec_vec_file_result: Result<Vec<Vec<String>>, serde_json::Error>
            = serde_json::from_str(verb_url_vec_content.as_str());


    let verb_url_vec_vec: Vec<Vec<String>> = match verb_url_vec_vec_file_result {
        Ok(verb_url_vec_vec) => verb_url_vec_vec,

        Err(_) => {
            let url_listing_vec_vec = generate_url_listing_vec_vec(&language_vec);
            println!("url_listing_vec_vec: {:?}", url_listing_vec_vec);

            let verb_vec_vec: Vec<Vec<String>> = scrape_url_listing_vec_vec(url_listing_vec_vec, backoff_duration, error_429_backoff_duration);

            let verb_url_vec_vec: Vec<Vec<String>> = generate_verb_url_vec_vec(verb_vec_vec, language_vec);
            save_string_vec_vec_to_file(&verb_url_vec_vec, "temp/json/verb_urls.json");

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

fn scrape_url_listing_vec_vec(url_listing_vec_vec: Vec<Vec<String>>, backoff_duration: Duration, error_429_backoff_duration: Duration) -> Vec<Vec<String>> {
    let mut verb_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, url_listing_vec) in url_listing_vec_vec.into_iter().enumerate() {
        for url_listing in url_listing_vec {
            let mut content: String = String::new();
            let response = reqwest::blocking::get(url_listing).unwrap().text().unwrap();

            // validate response
            // if error_429_backoff then increase and save backoff and wait error_429_backoff amount of seconds
            // if error_429_backoff then increase error_429_backoff and wait error_429_backoff amount of seconds
            match is_response_valid(&response) {
                Ok(res) => if res == false {
                    panic!("invalid response")
                },
                Err(err) => panic!("{}", err),
            };

            content.push_str(response.as_str());
            let document = scraper::Html::parse_document(&content);
            
            let section_container = scraper::Selector::parse("div.index-content>ol").unwrap();
            let section = document.select(&section_container).next().unwrap();

            // map to get the vec of verbs
            let li_selector = scraper::Selector::parse("li").unwrap();
            let mut verb_vec: Vec<String> = section.select(&li_selector).map(|li| li.text().collect::<String>()).collect::<Vec<String>>();
            verb_vec_vec[index].append(&mut verb_vec);

            // wait the backoff duration
            thread::sleep(backoff_duration);
        }
    }

    return verb_vec_vec;
}


// make the error an actual error and not a string so it can be matched
// match error_429 then wait and repeat else panic
fn is_response_valid(response: &str) -> Result<bool, &str> {

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


// take a page such as https://conjugator.reverso.net/conjugation-french-verb-faire.html
// and turn get all the data out of it and put into the PageInfo struct
// inner vec for each verb in a language
// outer vec for each language
fn generate_verb_page_info_vec_vec(verb_url_vec_vec: Vec<Vec<String>>, backoff: Duration, error_429_backoff: Duration) -> Vec<Vec<PageInfo>> {
    let mut verb_page_info_vec_vec: Vec<Vec<PageInfo>> = Vec::new();



    return verb_page_info_vec_vec;
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
                let tense: Vec<String> = match tense_data {
                    Tense { major: Some(_), minor: Some(_) } => {
                        Vec::from([language.clone(), tense_data.major.unwrap(), tense_data.minor.unwrap()])
                    },

                    Tense { major: Some(_), minor: None } => {
                        Vec::from([language.clone(), tense_data.major.unwrap(), String::from("")])
                    }
                
                    Tense { major: None, minor: None } => {
                        Vec::from([language.clone(), String::from(""), String::from("")])
                    },

                    Tense { major: None, minor: Some(_) } => {
                        Vec::from([language.clone(), String::from(""), tense_data.minor.unwrap()])
                    },
                };

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
            let rank: String = verb_page_info.metadata.rank.clone();
            let phrases: Vec<Phrase> = verb_page_info.phrases.clone();

            for phrase in phrases {
                for (index, subject) in phrase.subjects.into_iter().enumerate() {
                    // make tense_pk_map_vec a Vec<BTreeMap<Tense, i64>>?
                    let tense = tense_pk_map_vec[index].get(&(verb_page_info.tenses[index].clone().major.unwrap()+ " " + verb_page_info.tenses[index].clone().minor.unwrap().as_str())).unwrap().to_string();
                    let auxiliary = auxiliary_pk_map_vec[index].get(&phrase.auxiliaries[index].clone()).unwrap().to_string();
                    let conjugate = conjugate_pk_map_vec[index].get(&phrase.conjugates[index].clone()).unwrap().to_string();

                    let conjugation_data_vec: Vec<String> = Vec::from([rank.clone(), tense, subject, auxiliary, conjugate]);
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

