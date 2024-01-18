// test languages
#[cfg(test)]
// use super::super::functionality::languages::is_vector_valid;
use crate::functionality::languages::*;

#[test]
fn languages_is_vector_valid() {
    let valid_vec_1: Vec<String> = Vec::from([String::from("English"), String::from("French"), String::from("Italian"), String::from("Portuguese"), String::from("Spanish")]);
    let valid_vec_2: Vec<String> = Vec::from([String::from("Portuguese")]);
  
    let invalid_vec_1: Vec<String> = Vec::from([String::from("")]);
    let invalid_vec_2: Vec<String> = Vec::new();
    let invalid_vec_3: Vec<String> = Vec::from([String::from("English"), String::from("Italian"), String::from("English")]);
  
    // Should check if it is a real language as well
  
    assert!(is_vector_valid(&valid_vec_1).ok());
    assert!(is_vector_valid(&valid_vec_2).ok());
    assert_eq!(is_vector_valid(&invalid_vec_1), "Vector has duplicated languages");
    assert_eq!(is_vector_valid(&invalid_vec_2), "Vector has null element(s)");
    assert_eq!(is_vector_valid(&invalid_vec_3), "Vector has null element(s)");
}
