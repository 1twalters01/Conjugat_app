use::std::fs::File;
use std::io::Read;
use crate::crud::{open_file, append_file, delete_file};

pub fn scrape_model(languages: Vec<&str>) {
    for language in languages {
        let url: String = form_url(language);
        println!("{}", url);

        let mut content: String = String::new();
        let file_path: String = "temp/models/".to_string() + language + ".txt";
        let mut file: File = open_file(file_path);
       
        // let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
        // content.push_str(response.as_str());
        // append_file(&mut file, content);

        file.read_to_string(&mut content);

        let section_container = scraper::Selector::parse("div.model-contents").unwrap();

        let model_selector = scraper::Selector::parse("a[class=model-title-verb]").unwrap();
        // let group_selector = scraper::Selector::parse("div[class=column col-s]").unwrap();
        let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap(); 
        let group_selector = scraper::Selector::parse("p[class=group]").unwrap(); 

        let mut model: Vec<String> = Vec::new();

        let document = scraper::Html::parse_document(&content);

        for section in document.select(&section_container) {

            for model_scraped in section.select(&model_selector) {
                let model_a = model_scraped.text().collect::<Vec<_>>();
                let model_content = model_a[0].to_string();
                println!("{:?}", model_content);
                model.push(model_content);
            }

            // for group_scraped in section.select(&group_selector) {
            //     let group_p = group_scraped.text().collect::<Vec<_>>();
            //     // do match instead as [0] may not exist
            //     let group_content = group_p[0].to_string();
            //     println!("{:?}", group_content);
            // }

            // for ending_scraped in section.select(&ending_selector) {
            //     let ending_p = ending_scraped.text().collect::<Vec<_>>();
            //     let ending_content = ending_p[0].to_string();
            //     // println!("{:?}", ending_content.trim());
            // }
        }

        println!("{:?}", model);
        
        let json_file_path: String = "temp/json/models/".to_string() + language + ".json";
        delete_file(json_file_path.clone());
        let mut json_file: File = open_file(json_file_path);

        let mut model_json: String = String::from("[");
        for verb in model {
            model_json.push_str("
{
\"model\": \"verbs.group\",
\"pk\": int,
\"fields\": {
  \"group\": \"string\",
  \"language\": \"int\"
},");
        }
        model_json.pop();
        model_json.push_str("\n]");
        append_file(&mut json_file, model_json);
    }
}

fn form_url(language: &str) -> String {
    let url = "https://conjugator.reverso.net/conjugation-rules-model-".to_string() + language + ".html";
    return url.to_string()
}
