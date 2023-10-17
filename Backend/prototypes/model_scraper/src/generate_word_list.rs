use crate::crud::{append_file, delete_file, open_file};
use std::io::Read;
use std::fs::File;


pub fn generate_word_list_files(languages: Vec<&str>) {

    for (index, language) in languages.iter().enumerate() {
        let mut content:String = String::new();

        let file_path: String = "./temp/urls/".to_owned() + languages[index] + " urls.txt";
        let mut file = open_file(file_path);

        let mut urls: String = String::new();
        file.read_to_string(&mut urls);

        let url_arr = urls.split("\n");
        for url in url_arr {
            content.push_str(&extract_verbs(url));
        }

        let file_path: String = "./temp/base/".to_owned() + language + " word list.txt";
        delete_file(file_path.clone());

        let mut file: File = open_file(file_path);
        append_file(&mut file, content.to_owned());
    }
}



fn extract_verbs(common_verb_list_url: &str) -> String {
    let response: String = reqwest::blocking::get(common_verb_list_url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let class_selector = scraper::Selector::parse("div.index-content").unwrap();
    let verb_selector = scraper::Selector::parse("li>a").unwrap();
    
    let mut content: String = String::new();

    for section in document.select(&class_selector) {
        for verb in section.select(&verb_selector) {
            let verbs = verb.text().collect::<Vec<_>>();
            content.push_str(&(verbs[0].to_string() + "\n"));
            // println!("{content}");
        }
    }
    content.pop();
    return content
}

