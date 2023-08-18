#![allow(unused)]

mod generate_urls;
mod crud;

use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};
use crate::generate_urls::generate_url_text_file;

fn main() {
    generate_url_text_file();

    // let response = scrape_html("https://conjugator.reverso.net/index-spanish-1-250.html");
    // println!("{}", response);
}


// fn scrape_html(url: &str) -> String {
    // return reqwest::blocking::get(url).unwrap().text().unwrap();
// }
