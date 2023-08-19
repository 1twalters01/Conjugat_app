
use crate::crud::{append_file, delete_file, open_file};
use std::fs::{self, OpenOptions, File};


pub fn generate_url_text_files(languages: Vec<&str>) {
    let mut urls:Vec<Vec<String>> = generate_url_links(languages.clone());

    for (index, url_vec) in urls.iter().enumerate() {

        let file_path: String = "./temp/urls/".to_owned() + languages[index] + " urls.txt";
        delete_file(file_path.clone());

        let mut file: File = open_file(file_path);

        let url_vec_len = url_vec.len();
        for (index2, url) in url_vec.iter().enumerate() {
            let mut content = url.clone();
            if (index2+1) != url_vec_len {
                content = content + "\n";
            }
            append_file(&mut file, content.to_owned());
        }
    }
}


fn generate_url_links(languages: Vec<&str>) -> Vec<Vec<String>> {
    let mut urls: Vec<Vec<String>> = vec![vec![String::new(); 8]; 5];

    for (index1, language) in languages.iter().enumerate() {
        let url_vec_length = urls[index1].len().clone();
        for (index2, url) in urls[index1].iter_mut().enumerate() {
            let first: usize = 250*(index2) + 1;
            let second: usize = 250* (index2+1);

            let mut var: String = "https://conjugator.reverso.net/index-".to_string() + language + "-" + first.to_string().as_str() + "-" + second.to_string().as_str() + ".html";

            *url = var;
        }
    }

    return urls; 
}

