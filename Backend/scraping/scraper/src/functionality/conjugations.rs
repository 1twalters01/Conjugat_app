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
    time::Duration,
    thread,
};




pub async fn run_conjugations_modules() {
    // get vectors for the languages, groups, endings, and models
    let languages_data = read_data_from_file("temp/json/languages/languages.json");
    let groups_data = read_data_from_file("temp/json/models/groups.json");
    let endings_data = read_data_from_file("temp/json/models/endings.json");
    let models_data = read_data_from_file("temp/json/models/models.json");

    let languages: Vec<&str> = extract_languages(languages_data);

    let verb_urls_vec: Vec<Vec<&str>> = form_verb_urls(languages);
    save_verb_urls(verb_urls_vec);

    // Get exponential back off
    let (exponential_backoff, error_429_backoff): i64 = read_exponential_backoff_values;
    
    for (language_id, verb_urls) in verb_urls_vec.into_iter().enumerate() {
        for url in verb_urls {
            // async_scrape_html_from_url(url: &str)
            let mut content: String = String::new();
            reqwest::get(url).await.unwrap().text().await.unwrap();
            content.push_str(response.as_str());

            let document = scraper::Html::parse_document(&content);
            
            // Scrape top bar of reverso website, aka model, auxiliaries and other forms
            let top_section_container = scraper::Selector::parse("div.alternate-versions").unwrap();
            let model_selector = scraper::Selector::parse("span[id=ch_lblModel]").unwrap();
            let auxiliary_type_selector = scraper::Selector::parse("span[id=ch_lblAuxiliary]>a").unwrap();
            let form_type_selector = scraper::Selector::parse("span[id=ch_lblAutreForm]>a").unwrap();
        
            let mut model: String = String::new();
            let mut auxiliary_types: Vec<&str> = Vec::new();
            let mut form_types: Vec<&str> = vec![infinitive];
        
            for mut section in document.select(&top_section_container) {
                model = section.select(&model_selector).flat_map(|el| el.text().collect::<String>());
                println!("model: {}", model);
        
                auxiliary_types = section.select(&auxiliary_type_selector).flat_map(|el| el.text()).collect::<Vec<&str>>();
                println!("auxiliary types: {:?}", auxiliary_types);
        
                form_types.extend(section.select(&form_type_selector).flat_map(|el| el.text()).collect::<Vec<&str>>());
                println!("form types: {:?}", form_types);
            }

            // Create vec of alternate urls
            let alt_urls: Vec<&str> = form_types.map(|el| String::from("https://conjugator.reverso.net/conjugation-") + languages[language_id] + el.replace(" ", "%20") + ".html");

            // Scrape alternate urls
            thread::sleep(Duration::from_millis(exponential_backoff));
            let alt_content: Vec<String> = alt_urls.map(|url| async_scrape_html_from_url(url));

            // Scrape lower section
            //let tense_type_selector = scraper:;Selector::parse("div.word-wrap-title>h4").unwrap();
            //let tense_main_selector = scraper::Selector::parse("").unwrap();
            let tense_selector = scraper::Selector::parse("div[mobile-title]>p").unwrap();
            let subject_selector = scraper::Selector::parse("i.graytxt").unwrap();
            let auxiliary_selector = scraper::Selector::parse("span#ch_lblAuxiliary>a").unwrap();
            let conjugate_selector = scraper::Selector::parse("i.verbtxt").unwrap();
            // 
        }
    }
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
