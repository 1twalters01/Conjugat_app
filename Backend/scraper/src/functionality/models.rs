// Todo
use crate::data_types::{
    json_data::JsonData,
    field::{
        Field,
        FieldOptions,
    },
    field_options::{
        LanguageField,
        GroupField,
        EndingField,
        ModelField,
    }
};

use scraper::{
    html::Select,
    Html
};

use crate::helper_functions::{
    create_json_data_vec,
    save_data_to_json_file,
    // create_pool_connection,
    read_html_from_file,
    // async_scrape_html_from_url,
};

use std::collections::BTreeMap;
// use std::{
//     collections::HashSet,
//     result,
// };




pub async fn run_model_module() {
    // Read languages data from file
    let language_content: String = read_html_from_file("temp/json/languages/languages.json");
    let (language_data_vec, language_vec) = read_language_data_from_json_data(language_content.as_str());
    let language_pk_map_vec: Vec<BTreeMap<String, i64>> = get_language_pk_map_vec(&language_data_vec, &language_vec);


    // Read html for each language's model page on Reverso (saved for convenience)
    let content_vec: Vec<String> = get_model_html_vec(language_vec);


    // Parse html for each language's model page
    let document_vec: Vec<Html> = content_vec.into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data: Vec<Select> = get_main_data(&document_vec);


    // Create group data and group:language map
        // Where column 0: language, column 1: group
    let group_data_vec_vec: Vec<Vec<String>> = get_group_data_vec_vec(&main_data, &language_pk_map_vec);
    let group_data_vec: Vec<JsonData> = create_json_data_vec(group_data_vec_vec, FieldOptions::GroupField);
    save_data_to_json_file(&group_data_vec, "temp/json/models/groups.json");
    let group_language_id_map_vec: Vec<BTreeMap<String, i64>> = get_group_language_id_map_vec(group_data_vec);
    save_group_language_id_map_vec(&group_language_id_map_vec, "temp/json/models/language_group_btreemap.json");


    // Create ending data and ending:group map
        // Where 0: group, 1: ending
    let ending_data_vec_vec: Vec<Vec<String>> = get_ending_data_vec(&main_data, &group_language_id_map_vec);
    let ending_data_vec: Vec<JsonData> = create_json_data_vec(ending_data_vec_vec, FieldOptions::EndingField);
    save_data_to_json_file(&ending_data_vec, "temp/json/models/endings.json");
    let ending_group_id_map_vec_vec: Vec<Vec<BTreeMap<String, i64>>> = get_ending_group_id_map_vec_vec(ending_data_vec);
    save_ending_group_id_map_vec_vec(&ending_group_id_map_vec_vec, "temp/json/models/language_group_btreemap.json");


    // Create model data and model:ending map
        // Where 0: ending, 1: model
    let model_data_vec_vec: Vec<Vec<String>> = get_model_data_vec_vec(&document_vec, &ending_group_id_map_vec_vec);
    let model_data_vec: Vec<JsonData> = create_json_data_vec(model_data_vec_vec, FieldOptions::ModelField);
    save_data_to_json_file(&model_data_vec, "temp/json/models/models.json");
    let model_ending_id_map_vec_vec: Vec<Vec<BTreeMap<String, String>>> = get_model_ending_id_map_vec_vec(model_data_vec);
    save_model_ending_id_map_vec_vec(&model_ending_id_map_vec_vec, "temp/json/models/language_group_btreemap.json");


    // save_data_to_postgres(&groups_data, &endings_data, &models_data).await;
}


fn read_language_data_from_json_data(language_content: &str) -> (Vec<JsonData>, Vec<String>) {
    let language_data_vec: Vec<JsonData> = serde_json::from_str(language_content).unwrap();
    let mut language_vec: Vec<String> = Vec::new();

    for language_data in &language_data_vec {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            language_vec.push(language.clone());
        }
    }

    return (language_data_vec, language_vec);
}


fn get_language_pk_map_vec(language_data_vec: &Vec<JsonData>, language_vec: &Vec<String>) -> Vec<BTreeMap<String, i64>> {
    let mut language_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new();

    for language_data in language_data_vec {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            let mut language_pk_map: BTreeMap<String, i64> = BTreeMap::new();
            language_pk_map.insert(language.clone(), language_data.pk);
            language_pk_map_vec.push(language_pk_map);
        }
    }

    return language_pk_map_vec;
}


fn get_model_html_vec(language_vec: Vec<String>) -> Vec<String> {
    let mut content_vec: Vec<String> = Vec::new();
    
    for language in language_vec {
        let file_path: String = String::from("temp/html/models/") + language.to_lowercase().as_str() + ".txt";
        let content: String = read_html_from_file(file_path.as_str());
        content_vec.push(content);
    }

    return content_vec;
}


fn get_main_data(document_vec: &Vec<Html>) -> Vec<Select> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let main_data = document_vec.iter().enumerate()
        .map(|(index, document)| document.select(&main_selector)).collect::<Vec<_>>();
    
    return main_data;
}


fn get_group_data_vec_vec(main_data_vec: &Vec<Select>, language_data: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut group_data_vec_vec: Vec<Vec<String>> = Vec::new();
   
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut groups: Vec<&str> = main_data.into_iter()
            .map(|data| data.select(&group_selector).next().unwrap().text().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .iter().filter(|testvec| testvec.len() > 0)
            .map(|testvec| testvec[0]).collect::<Vec<_>>();

        groups.sort();
        groups.dedup();

        for group in groups {
            let group_vec: Vec<String> = vec![index.to_string(), group.to_string()];
            group_data_vec_vec.push(group_vec);
        }
    }

    return group_data_vec_vec;
}


fn get_group_language_id_map_vec(group_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // Outer vec by language
    // map os <group, language>
    let mut group_language_id_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for group_data in group_data_vec {
        let mut group_language_id_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_language_id_map.insert(group.to_owned(), language_id);
            if language_id > group_language_id_map_vec.len().to_string().parse::<i64>().unwrap() {
                group_language_id_map_vec.push(group_language_id_map);
            } else {
                group_language_id_map_vec[language_id.to_string().parse::<usize>().unwrap()].insert(group.to_owned(), language_id);
            }
        }
    }

    return group_language_id_map_vec;
}


fn save_group_language_id_map_vec(group_language_id_map_vec_vec: &Vec<BTreeMap<String, i64>>, file_path: &str) {
    println!("group_language_id_map_vec_vec: {:?}", group_language_id_map_vec_vec);
    println!("file_path: {}", file_path);
}


fn get_ending_data_vec(main_data_vec: &Vec<Select>, group_language_id_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut ending_data_vec_vec: Vec<Vec<String>> = Vec::new();
    let mut ending_count: i64 = 0;
    
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut ending_group_array_vec = main_data.into_iter()
            .map(|data| vec![data.select(&ending_selector).next()
                            .unwrap().text().collect::<Vec<&str>>(),
                         data.select(&group_selector).next()
                            .unwrap().text().collect::<Vec<&str>>()])
            .collect::<Vec<_>>();
            
            for ending_group_array in ending_group_array_vec.iter_mut() {
                for item in ending_group_array.iter_mut() {
                    if item[1] == "" {item[1] = group_language_id_map_vec[index].get("-").unwrap().to_string().as_str()}
                    else {item[1] = group_language_id_map_vec[index].get(item[1]).unwrap().to_string().as_str()}
                }     
            }

        ending_group_array_vec.sort();
        ending_group_array_vec.dedup();
       
        for (index, ending_group) in ending_group_array_vec.into_iter().enumerate() {
            let ending_data_vec: Vec<String> = vec![
                ending_group[index][0].to_string(),
                ending_group[index][1].to_string()
            ];

            ending_data_vec_vec.push(ending_data_vec);
        }
        // endings_groups_vec.sort();
        // endings_groups_vec.dedup();
    }

    return ending_data_vec_vec;
}


fn get_ending_group_id_map_vec_vec(ending_data_vec: Vec<JsonData>) -> Vec<Vec<BTreeMap<String, i64>>> {
    // map os <ending, group>
    let mut ending_vec_vec_map: Vec<Vec<BTreeMap<String, i64>>> = Vec::new(); 
    for ending_data in ending_data_vec {
        let mut ending_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();
           ending_map.insert(ending.to_owned(), group_id);
            if group_id > ending_vec_vec_map.len().to_string().parse::<i64>().unwrap() {
                ending_vec_vec_map.push(Vec::from([ending_map]));
            } else {
                ending_vec_vec_map[group_id.to_string().parse::<usize>().unwrap()].push(ending_map);
            }
        }
    }

    return ending_vec_vec_map;
}

    
fn save_ending_group_id_map_vec_vec(ending_group_id_map_vec_vec: &Vec<Vec<BTreeMap<String, i64>>>, file_path: &str) {
    println!("ending_group_id_map_vec_vec: {:?}", ending_group_id_map_vec_vec);
    println!("file_path: {}", file_path);
}


fn get_model_data_vec_vec(document_vec: &Vec<Html>, ending_group_map_vec_vec: &Vec<Vec<BTreeMap<String, i64>>> ) -> Vec<Vec<String>> {
    let all_selector = scraper::Selector::parse("div.model-row").unwrap();
    let model_selector = scraper:: Selector::parse("").unwrap();
    let ending_selector = scraper::Selector::parse("").unwrap();
    let mut models_data_vec_vec: Vec<Vec<String>> = Vec::new();
    let mut model_count: i64 = 0;

    for (index, document) in document_vec.into_iter().enumerate() {
        let mut models_endings_vec = document.select(&all_selector).into_iter()
            .map(|all_scraped| [all_scraped.select(model_selector).next.unwrap().text.collect::<Vec<_>>(),
                                all_scraped.select(&ending_selector).next.unwrap().text.collect::<Vec<_>>()]) // use map to turn word into int
            .collect::<Vec<_>>()
            .iter().filter() // Make all empty groups be equal to "-"
            .map(|testvecvec| testvecvec.map(|testvec| testvec[0]).collect::<Vec<_>>()).collect::<Vec<_>>();

        models_endings_vec.sort();
        models_endings_vec.dudup();
    }

    return models_data_vec_vec;
}


fn get_model_ending_id_map_vec_vec(model_data_vec: Vec<JsonData>) -> Vec<Vec<BTreeMap<String, String>>> {
    // map os <model, ending>
    let mut model_vec_vec_map: Vec<Vec<BTreeMap<String, String>>> = Vec::new(); 
    for model_data in model_data_vec {
        let mut model_map: BTreeMap<String, String> = BTreeMap::new();
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();
           ending_map.insert(ending.to_owned(), group_id);
            if group_id > ending_vec_vec_map.len().to_string().parse::<i64>().unwrap() {
                ending_vec_vec_map.push(Vec::from([ending_map]));
            } else {
                ending_vec_vec_map[group_id.to_string().parse::<usize>().unwrap()].push(ending_map);
            }
        }
    }

    return model_vec_vec_map;
}


fn save_model_ending_id_map_vec_vec(ending_model_map_vec_vec: &Vec<Vec<BTreeMap<String, String>>>, file_path: &str) {
    println!("ending_model_map_vec_vec: {:?}", ending_model_map_vec_vec);
    println!("file_path: {}", file_path);
}


// async fn save_data_to_postgres(groups_data: &Vec<JsonData>, endings_data: &Vec<JsonData>, models_data: &Vec<JsonData>) {
//     let pool = create_pool_connection().await;
//
//     for group_data in groups_data {
//         println!("{:?}, {:?}", group_data, group_data.pk);
//         if let Field::GroupField(GroupField{language, group}) = &group_data.fields {
//
//             //if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
//                 .bind(group_data.pk)
//                 .bind(language)
//                 .bind(group)
//                 .execute(&pool)
//                 .await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
//                         .bind(language)
//                         .bind(group)
//                         .bind(group_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//
//         } else {
//             panic!("non-group in group field");
//         };
//     }
//
//
//     for ending_data in endings_data {
//         if let Field::EndingField(EndingField { group, ending }) = &ending_data.fields {
//             // if unable to insert into table then update table else panic
//             let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
//                 .bind(ending_data.pk)
//                 .bind(group)
//                 .bind(ending)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_ending SET group=($1), ending=($2), WHERE id=($3)")
//                         .bind(group)
//                         .bind(ending)
//                         .bind(ending_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 }
//             };
//         } else {
//             panic!("non-ending in ending field");
//         };
//     }
//     
//
//     for model_data in models_data {
//         println!("{:?} {:?}", model_data, model_data.pk);
//         if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
//             let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
//                 .bind(model_data.pk)
//                 .bind(ending)
//                 .bind(model)
//                 .execute(&pool).await;
//
//             match insert_query {
//                 Ok(res) => res,
//                 Err(_) => {
//                     let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
//                         .bind(ending)
//                         .bind(model)
//                         .bind(model_data.pk)
//                         .execute(&pool).await;
//
//                     let rewrite_result = match rewrite_query {
//                         Ok(res) => res,
//                         Err(err) => panic!("Error: {:?}", err),
//                     };
//                     rewrite_result
//                 },
//             };
//         } else {
//             panic!("non-model in model field");
//         };
//     }
// }
