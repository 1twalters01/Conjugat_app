use crate::{
    data_types::page_info::PageInfo,
    helper_functions::{read_functions::read_file_to_string, save_functions::save_data_to_file},
};

pub fn run_resurrection_module() {
    let base_vec_file_path = "temp/json/conjugations/English_scraped_verb_vec.json";
    let page_info_vec_vec_string =
        read_file_to_string("temp/json/conjugations/English_page_info.json");
    let page_info_vec: Vec<PageInfo> = serde_json::from_str(&page_info_vec_vec_string).unwrap();

    let mut base_vec: Vec<String> = Vec::new();
    for page_info in page_info_vec {
        let base = page_info.metadata.base;
        base_vec.push(base);
        println!("{:?}", base_vec);
    }

    save_data_to_file(&mut base_vec, base_vec_file_path);
}
