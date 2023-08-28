use std::fs::{self, OpenOptions, File};
use crate::crud::{append_file, delete_file, open_file};
use std::io::{ErrorKind, Read, Write};

pub fn scrape_verb(infinitive: &str, language: &str) {
    let content: String = obtain_html_string(infinitive, language);

    // generate vectors for variables
    let mut auxiliary: String = String::new();
    let mut model: String = String::new();
    let mut tenses: Vec<String> = Vec::new();
    let mut subjects: Vec<String> = Vec::new();
    let mut auxiliaries: Vec<String> = Vec::new();
    let mut conjugates: Vec<String> = Vec::new();
    let mut main: Vec<Vec<String>> = Vec::new();


    let model_selector = scraper::Selector::parse("span#ch_lblModel>a").unwrap();
    let auxiliary_selector = scraper::Selector::parse("span#ch_lblAuxiliary>a").unwrap();

    let section_container = scraper::Selector::parse("div.word-wrap-row").unwrap();
    let tense_selector = scraper::Selector::parse("div[mobile-title]>p").unwrap();
    let subject_selector = scraper::Selector::parse("i.graytxt").unwrap();
    let conjugate_selector = scraper::Selector::parse("i.verbtxt").unwrap();
    let main_container = scraper::Selector::parse("li").unwrap();
    let main_selector = scraper::Selector::parse("i").unwrap();


    let document = scraper::Html::parse_document(&content);
    let mut tense_h4: String = String::new();

    for (index, section) in document.select(&section_container).enumerate() {
        // let mut tense_h4:Vec<&str> = Vec::new();
        let h4_selector = scraper::Selector::parse("div.word-wrap-title>h4").unwrap(); 

        for main_tense in section.select(&h4_selector) {
            tense_h4 = main_tense.text().collect::<String>();
        }

        for tense_scraped in section.select(&tense_selector) {
            let tense_p = tense_scraped.text().collect::<Vec<_>>();
            let tense_content = tense_h4.clone().to_owned() + " " + tense_p[0];
            tenses.push(tense_content);
        }



        for subject_scraped in section.select(&subject_selector) {
            let subject_i = subject_scraped.text().collect::<Vec<_>>();
            let subject_content = subject_i[0].to_string();
            if !(subjects.contains(&subject_content)) {
                subjects.push(subject_content);
            }
        }



        for conjugate_scraped in section.select(&conjugate_selector) {
            let conjugate_i = conjugate_scraped.text().collect::<Vec<_>>();
            let conjugate_content = conjugate_i[0].to_string();
            if !(conjugates.contains(&conjugate_content)) {
                conjugates.push(conjugate_content);
            }
        }

        for main_section in section.select(&main_container) {
            let mut main_vec: Vec<String> = Vec::new();
            for (index, main_scraped) in main_section.select(&main_selector).into_iter().enumerate() {
                let main_i = main_scraped.text().collect::<Vec<_>>();
                let main_content = main_i[0].to_string();
                main_vec.push(main_content);
            }
            main.push(main_vec);
        }
    }

    for model_scraped in document.select(&model_selector) {
        let model_a = model_scraped.text().collect::<Vec<_>>();
        let model_content = model_a[0].to_string();
        model = model_content;
        break;
    }

    for auxiliary_scraped in document.select(&auxiliary_selector) {
        let auxiliary_a = auxiliary_scraped.text().collect::<Vec<_>>();
        let auxiliary_content = auxiliary_a[0].to_string();
        auxiliary = auxiliary_content;
        break;
    }

    println!("\n infinitive: {:?}", infinitive);
    println!("\n language: {:?}", language);
    println!("\n model: {:?}", model);
    println!("\n auxiliary: {:?}", auxiliary);
    println!("\n tenses: {:?}", tenses);
    println!("\n subjects: {:?}", subjects);
    println!("\n conjugates: {:?}", conjugates);
    println!("\n main: {:?}", main);
}



fn obtain_html_string(infinitive: &str, language: &str) -> String {
    let url: String = form_url(infinitive, language);
    // println!("{}", url);

    let mut content: String = String::new();
    let file_path: String = "temp/".to_string() + infinitive + " html.txt";

    let mut file: File = open_file(file_path);

    // let mut response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    // content.push_str(response.as_str());
    // append_file(&mut file, content);

    file.read_to_string(&mut content);

    return content;
}



fn form_url(infinitive: &str, language: &str) -> String {
    let url: String =  "https://conjugator.reverso.net/conjugation-".to_string() + language + "-verb-" + infinitive + ".html";
    return url;
}

