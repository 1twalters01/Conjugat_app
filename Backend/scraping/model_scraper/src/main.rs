#![allow(unused)]

mod crud;
mod generate_urls;
mod generate_word_list;

use std::fs::{self, OpenOptions, File};
use std::io::{ErrorKind, Read, Write};
use crate::generate_urls::generate_url_text_files;
use crate::generate_word_list::generate_word_list_files;

use crate::crud::{append_file, delete_file, open_file};



fn main() {
    let languages: Vec<&str> = vec!["Spanish", "Portuguese", "Italian", "French", "English"];

    // generate_url_text_files(languages.clone());
    // println!("url list has been generated");

    generate_word_list_files(languages);
    println!("word list has been generated");

}




