
use crate::crud::{append_file, delete_file, open_file};
use std::fs::{self, OpenOptions, File};


pub fn generate_url_text_file() {
    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English"];

    let mut urls:Vec<Vec<String>> = generate_url_links(languages.clone());

    for (index, set) in urls.iter().enumerate() {

        let file_path: String = "./temp/urls/".to_owned() + languages[index] + " urls.txt";
        delete_file(file_path.clone());

        let mut file: File = open_file(file_path);

        for url in set.iter() {
            append_file(&mut file, url.to_owned());
        }
    }
}


fn generate_url_links(languages: Vec<&str>) -> Vec<Vec<String>> {
    let mut urls: Vec<Vec<String>> = vec![vec![String::new(); 8]; 5];

    for (index1, language) in languages.iter().enumerate() {
        for (index2, url) in urls[index1].iter_mut().enumerate() {
            let first: usize = 250*(index2) + 1;
            let second: usize = 250* (index2+1);

            let var: String = "https://conjugator.reverso.net/index-".to_string() + language + "-" + first.to_string().as_str() + "-" + second.to_string().as_str() + ".html";
            *url = var;
        }
    }

    return urls; 
}

