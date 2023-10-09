// Todo
use crate::data_types::JsonData::{
    JsonData,
    Field,
    FieldOptions,
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
};

use crate::helper_functions::{
    create_json_data_vec,
    save_data_to_json_file,
    create_pool_connection,
};

use std::{
    collections::HashSet,
    result,
};




pub async fn run_model_module() {
    // get html vector for the models of each language saved
    let (languages_data, languages) = read_languages();
    let content_vec: Vec<String> = get_model_html_vec(languages);


    // 0:language, 1: group
    let groups_data_vec_vec: Vec<Vec<&str>> = get_groups_data_vec_vec(content_vec);
    // 0: group, 1: ending
    let endings_data_vec_vec: Vec<Vec<&str>> = get_endings_data_vec_vec();
    // 0: ending, 1: model
    let models_data_vec_vec: Vec<Vec<&str>> = get_models_data_vec_vec();

    let groups_data: Vec<JsonData> = create_json_data_vec(group_data_vec_vec, FieldOptions::GroupField);
    let endings_data: Vec<JsonData> = create_json_data_vec(ending_data_vec_vec, FieldOptions::EndingField);
    let model_data: Vec<JsonData> = create_json_data_vec(model_data_vec_vec, FieldOptions::ModelField);

    let groups_file_path: String = "temp/json/models/groups.json".to_string();
    save_data_to_json_file(&groups_data, groups_file_path);
    let endings_file_path: String = "temp/json/models/endings.json".to_string();
    save_data_to_json_file(&endings_data, endings_file_path);
    let models_file_path: String = "temp/json/models/models.json".to_string();
    save_data_to_json_file(&models_data, models_file_path);

    save_to_postgres(&groups_data, &endings_data, &models_data).await;
}


fn read_languages() -> Vec<String> {
    // read languages from json file
    let language_file_path: &str = "temp/json/models/groups.json";
    let mut language_content: String = read_html_from_file(language_file_path;
    let languages_data: Vec<JsonData> = serde_json::from_str(language_content.as_str()).unwrap();
    
    // create urls
    let languages: Vec<&str> = Vec::new();
    for language_data in languages_data {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            languages.push(language);
        }
    }
    return language_data, language
}

fn get_model_html_vec(languages: Vec<&str>) {    
    let mut urls: Vec<String> = Vec::new();
    for language in languages {
        urls.push(String::from("https://conjugator.reverso.net/conjugation-rules-model-") + language + ".html");
    }

    // scrape the urls of their html
    let mut content_vec: Vec<String> = Vec::new();
    
    for url in urls {
        let content: String = scrape_html_from_url(url.as_str())
        content_vec.push(content);
    }
    
    return content_vec;
}


fn get_groups_data_vec_vec(content_vec: Vec<String>, languages) {
    let group_selector = scraper::Selector::parse("a[class=group]").unwrap();
    let groups_data_vec_vec: Vec<Vec<&str>> = Vec::new();
    
    for (index, extract) in content_vec.into_iter().enumerate() {
        let document = scraper::Html::parse_document(&extract);
        groups = document.select(&group_selector).flat_map(|el| el.text()).collect::<Vec<&str>>();
        
        // for section in document.select(&section_container) {
            // group_vec = section.select(&group_selector).flat_map(|el| el.text()).collect::<Vec<&str>>();
            // println!("model: {}", model);
        // }
        groups.sort()
        
        for group in groups {
            let group_vec: vec<&str> = vec![languages[index], group];
            groups_data_vec_vec.push(group_vec);
        }
    }

    return groups_data_vec_vec;
}


fn get_endings_data_vec_vec(content_vec: Vec<String>) {
    
}

fn get_endings_data_vec_vec(content_vec: Vec<String>) {
    
}



async fn save_data_to_postgres(groups_data: &Vec<JsonData>, endings_data: &Vec<JsonData>, models_data: &Vec<JsonData>) {
    pool = create_pool_connection();

    for group_data in groups_data {
        println!("{:?}, {:?}", group_data, group_data.pk);
        if let Field::GroupField(GroupField{language, group}) = &group_data.fields {

            //if unable to insert into table then update table else panic
            let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
                .bind(group_data.pk)
                .bind(language)
                .bind(group)
                .execute(&pool)
                .await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
                        .bind(language)
                        .bind(group)
                        .bind(group_data.pk)
                        .execute(&pool).await;

                    let rewrite_result = match rewrite_query {
                        Ok(res) => res,
                        Err(err) => panic!("Error: {:?}", err),
                    };
                    rewrite_result
                },
            };

        } else {
            panic!("non-group in group field");
        };
    }


    for ending_data in endings_data {
        if let Field::EndingField(EndingField { group, ending }) = &ending_data.fields {
            // if unable to insert into table then update table else panic
            let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
                .bind(ending_data.pk)
                .bind(group)
                .bind(ending)
                .execute(&pool).await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_ending SET group=($1), ending=($2), WHERE id=($3)")
                        .bind(group)
                        .bind(ending)
                        .bind(ending_data.pk)
                        .execute(&pool).await;

                    let rewrite_result = match rewrite_query {
                        Ok(res) => res,
                        Err(err) => panic!("Error: {:?}", err),
                    };
                    rewrite_result
                }
            };
        } else {
            panic!("non-ending in ending field");
        };
    }
    

    for model_data in endings_data {
        println!("{:?} {:?}", model_data, model_data.pk);
        if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
            let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
                .bind(model_data.pk)
                .bind(ending)
                .bind(model)
                .execute(&pool).await;

            let insert_result = match insert_query {
                Ok(res) => res,
                Err(err) => {
                    let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
                        .bind(ending)
                        .bind(model)
                        .bind(model_data.pk)
                        .execute(&pool).await;

                    let rewrite_result = match rewrite_query {
                        Ok(res) => res,
                        Err(err) => panic!("Error: {:?}", err),
                    };
                    rewrite_result
                },
            };
        } else {
            panic!("non-model in model field");
        };
    }
}
