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

use crate::helper_functions::{
    create_json_data_vec,
    save_data_to_json_file,
    read_html_from_file,
    open_file,
    append_file,
    // create_pool_connection,
};

use scraper::Html;

use std::{
    collections::BTreeMap,
    fs
};


pub async fn run_model_module() {
    // Read languages data from file
    let language_content: String = read_html_from_file("temp/json/languages/languages.json");
    let (language_data_vec, language_vec) = read_language_data_from_json_data(language_content.as_str());
    let language_pk_map_vec: Vec<BTreeMap<String, i64>> = get_language_pk_map_vec(&language_data_vec, &language_vec);
    save_string_i64_map_vec(&language_pk_map_vec, "temp/languages/btreemaps/languages.json");

    // Read html for each language's model page on Reverso (saved for convenience)
    let content_vec: Vec<String> = get_model_html_vec(language_vec);


    // Create group data and group:language map
        // Where column 0: language, column 1: group
    let group_data_vec_vec: Vec<Vec<String>> = get_group_data_vec_vec(content_vec.clone(), &language_pk_map_vec);
    let group_json_data_vec: Vec<JsonData> = create_json_data_vec(group_data_vec_vec, FieldOptions::GroupField);
    save_data_to_json_file(&group_json_data_vec, "temp/json/models/groups.json");
    // println!("\ngroup_json_data_vec: {:#?}", group_json_data_vec);
    let group_language_id_map_vec: Vec<BTreeMap<String, i64>> = get_group_language_id_map_vec(group_json_data_vec.clone());
    save_string_i64_map_vec(&group_language_id_map_vec, "temp/json/models/btreemaps/group_language.json");
    // println!("group_language_id_map_vec: {:?}", group_language_id_map_vec);
    let group_pk_map_vec: Vec<BTreeMap<String, i64>> = get_group_pk_map_vec(group_json_data_vec);
    save_string_i64_map_vec(&group_pk_map_vec, "temp/json/models/btreemaps/groups.json");
    // println!("group_pk_map_vec: {:?}", group_pk_map_vec);


    // Create ending data and ending:group map
        // Where 0: group, 1: ending
    let ending_data_vec_vec: Vec<Vec<String>> = get_ending_data_vec(content_vec.clone(), &group_pk_map_vec);
    let ending_json_data_vec: Vec<JsonData> = create_json_data_vec(ending_data_vec_vec, FieldOptions::EndingField);
    // println!("\nending_json_data_vec: {:#?}", ending_json_data_vec);
    save_data_to_json_file(&ending_json_data_vec, "temp/json/models/endings.json");
    let ending_group_id_map_vec: Vec<BTreeMap<String, i64>> = get_ending_group_id_map_vec(ending_json_data_vec.clone());
    save_string_i64_map_vec(&ending_group_id_map_vec, "temp/json/models/btreemaps/ending_group.json");
    // println!("\nending_group_id_map_vec: {:?}", ending_group_id_map_vec);
    let ending_language_id_map_vec: Vec<BTreeMap<String, i64>> = get_ending_language_id_map_vec(ending_json_data_vec.clone(), &group_language_id_map_vec);
    save_string_i64_map_vec(&ending_language_id_map_vec, "temp/json/models/btreemaps/ending_language.json");
    let ending_pk_map_vec: Vec<BTreeMap<String, i64>> = get_ending_pk_map_vec(ending_json_data_vec.clone());
    save_string_i64_map_vec(&ending_pk_map_vec, "temp/json/models/btreemaps/endings.json");
    // println!("ending_pk_map_vec: {:?}", ending_pk_map_vec);


    // Create model data and model:ending map
        // Where 0: ending, 1: model
    let model_data_vec_vec: Vec<Vec<String>> = get_model_data_vec_vec(content_vec.clone(), &ending_pk_map_vec.clone(), &ending_group_id_map_vec, &group_pk_map_vec);
    let model_json_data_vec: Vec<JsonData> = create_json_data_vec(model_data_vec_vec, FieldOptions::ModelField);
    save_data_to_json_file(&model_json_data_vec, "temp/json/models/models.json");
    // println!("model_json_data_vec: {:#?}", model_json_data_vec);
    // let model_ending_id_map_vec: Vec<BTreeMap<String, i64>> = get_model_ending_id_map_vec_vec(model_json_data_vec.clone());
    // save_string_i64_map_vec(&model_ending_id_map_vec, "temp/json/models/btreemaps/model_ending.json");
    // println!("model_ending_id_map_vec: {:?}", model_ending_id_map_vec);
    let model_language_id_map_vec: Vec<BTreeMap<String, i64>> = get_model_language_id_map_vec(model_json_data_vec.clone(), &ending_language_id_map_vec);
    save_string_i64_map_vec(&model_language_id_map_vec, "temp/json/models/btreemaps/model_language.json");
    let model_pk_map_vec: Vec<BTreeMap<String, i64>> = get_model_pk_map_vec(model_json_data_vec);
    save_string_i64_map_vec(&model_pk_map_vec, "temp/json/models/btreemaps/models.json");
    // println!("model_pk_map_vec: {:?}", model_pk_map_vec);


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


fn get_language_pk_map_vec(language_data_vec: &Vec<JsonData>, _language_vec: &Vec<String>) -> Vec<BTreeMap<String, i64>> {
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

fn get_group_data_vec_vec(content_vec: Vec<String>, _language_pk_map: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    // Have to repeatedly have this or rust will complain:
        // trait is not fulfilled for main_data_vec to implement clone and I cba to implement
        // it (though I probably should)
        // cannot use &main_data_vec as that would make it a shared reference meaning you
        // can't use .into_iter on it

    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec.into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec.iter()
        .map(|document| document.select(&main_selector)).collect::<Vec<_>>();


    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut group_data_vec_vec: Vec<Vec<String>> = Vec::new();
   
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        // let mut group_vec: Vec<&str> = Vec::new();
    
        // for extract in main_data.into_iter() {
        // }

        let mut group_vec = main_data.into_iter()
            .map(|data| data.select(&group_selector).next().unwrap().text().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .iter().filter(|testvec| testvec.len() > 0)
            .map(|testvec| testvec[0]).collect::<Vec<_>>();

        group_vec.sort();
        group_vec.dedup();

        group_data_vec_vec.push(vec![index.to_string(), "-".to_string()]);
        for group in group_vec {
            let group_vec: Vec<String> = vec![index.to_string(), group.to_string()];
            group_data_vec_vec.push(group_vec);
        }
    }

    // group_data_vec_vec.sort();
    // group_data_vec_vec.dedup();

    return group_data_vec_vec;
}


fn get_group_language_id_map_vec(group_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // Outer vec by language
    // map os <group, language>
    let mut group_language_id_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for group_data in group_json_data_vec {
        let mut group_language_id_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_language_id_map.insert(group.to_owned(), language_id);

            if language_id >= group_language_id_map_vec.len().to_string().parse::<i64>().unwrap() {
                group_language_id_map_vec.push(group_language_id_map);
                group_language_id_map_vec[language_id.to_string().parse::<usize>().unwrap()].insert(String::from("-"), language_id);
            } else {
                group_language_id_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut group_language_id_map);
            }
        }
    }

    return group_language_id_map_vec;
}


fn get_group_pk_map_vec(group_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // map os <group, pk>
    let mut group_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for group_data in group_json_data_vec {
        let mut group_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_pk_map.insert(group.to_owned(), group_data.pk);
            if language_id >= group_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                group_pk_map_vec.push(group_pk_map);
            } else {
                group_pk_map_vec[language_id.to_string().parse::<usize>().unwrap()].append(&mut group_pk_map);
            }

        }
    }

    return group_pk_map_vec;
}


fn get_ending_data_vec(content_vec: Vec<String>, group_pk_map_vec: &Vec<BTreeMap<String, i64>>) -> Vec<Vec<String>> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec.into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec.iter()
        .map(|document| document.select(&main_selector)).collect::<Vec<_>>();


    let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut ending_data_vec_vec: Vec<Vec<String>> = Vec::new();
    
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut ending_group_array_vec: Vec<Vec<Vec<String>>> = main_data.into_iter()
            .map(|data| vec![data.select(&ending_selector).next()
                            .unwrap().text().collect::<Vec<&str>>()
                            .into_iter().map(|data| data.trim().to_string()).collect::<Vec<String>>(),
                         data.select(&group_selector).next()
                            .unwrap().text().collect::<Vec<&str>>()
                            .into_iter().map(|data| data.trim().to_string()).collect::<Vec<String>>(),
                         ])
            .collect::<Vec<_>>();

        // println!("group_pk_map_vec: {:?}", group_pk_map_vec);
        for ending_group_array in ending_group_array_vec.iter_mut() {
            if ending_group_array[1].len() == 0 {
                ending_group_array[1].push("-".to_string());
            }
            ending_group_array[1][0] = group_pk_map_vec[index].get(&ending_group_array[1][0]).unwrap().to_string()
        }

        ending_group_array_vec.sort();
        ending_group_array_vec.dedup();
       
        for ending_group in ending_group_array_vec.into_iter() {
            let ending_data_vec: Vec<String> = vec![
                ending_group[1][0].to_string(),
                ending_group[0][0].to_string()
            ];

            ending_data_vec_vec.push(ending_data_vec);
        }
    }

    ending_data_vec_vec.sort_by(|a, b| a[0].parse::<i64>().unwrap().cmp(&b[0].parse::<i64>().unwrap()));

    return ending_data_vec_vec;
}


fn get_ending_group_id_map_vec(ending_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // map os <ending, group>
    let mut ending_group_id_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for ending_data in ending_json_data_vec {
        let mut ending_group_id_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();
            ending_group_id_map.insert(ending.to_owned(), group_id);

            while group_id >= ending_group_id_map_vec.len().to_string().parse::<i64>().unwrap() {
                let mut initial_map: BTreeMap<String, i64> = BTreeMap::new();
                initial_map.insert(String::from("-"), ending_group_id_map_vec.len().to_string().parse::<i64>().unwrap());
                ending_group_id_map_vec.push(initial_map);
            }

            ending_group_id_map_vec[group_id.to_string().parse::<usize>().unwrap()].append(&mut ending_group_id_map);
        }
    }

    return ending_group_id_map_vec;
}

    


fn get_ending_pk_map_vec(ending_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // map os <ending, pk>
    let mut ending_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for ending_data in ending_json_data_vec {
        let mut ending_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();
            ending_pk_map.insert(ending.to_owned(), ending_data.pk);

            while group_id >= ending_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                let mut initial_map: BTreeMap<String, i64> = BTreeMap::new();
                initial_map.insert(String::from("-"), ending_pk_map_vec.len().to_string().parse::<i64>().unwrap());
                ending_pk_map_vec.push(initial_map);
            }
            ending_pk_map_vec[group_id.to_string().parse::<usize>().unwrap()].append(&mut ending_pk_map);
        }
    }

    return ending_pk_map_vec;
}




fn get_model_data_vec_vec(content_vec: Vec<String>, ending_pk_map_vec: &Vec<BTreeMap<String, i64>>, ending_group_id_map_vec: &Vec<BTreeMap<String, i64>>, group_pk_map_vec: &Vec<BTreeMap<String, i64>> ) -> Vec<Vec<String>> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec.into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec.iter()
        .map(|document| document.select(&main_selector)).collect::<Vec<_>>();


    let model_selector = scraper:: Selector::parse("a[class=model-title-verb]").unwrap();
    let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut model_data_vec_vec: Vec<Vec<String>> = Vec::new();
    
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut model_ending_array_vec: Vec<Vec<Vec<String>>> = main_data.into_iter()
            .map(|data| vec![data.select(&model_selector).next()
                                .unwrap().text().collect::<Vec<_>>()
                                .into_iter().map(|data| data.trim().to_string()).collect::<Vec<String>>(),
                            data.select(&ending_selector).next()
                                .unwrap().text().collect::<Vec<_>>() // use map to turn word into int
                                .into_iter().map(|data| data.trim().to_string()).collect::<Vec<String>>(),
                            data.select(&group_selector).next()
                                .unwrap().text().collect::<Vec<_>>() // use map to turn word into int
                                .into_iter().map(|data| data.trim().to_string()).collect::<Vec<String>>()
            ])

            .collect::<Vec<_>>();

        for model_ending_array in model_ending_array_vec.iter_mut() {
            if model_ending_array[1].len() == 0 {
                model_ending_array[1].push("-".to_string())
            }
            if model_ending_array[2].len() == 0 {
                model_ending_array[2].push("-".to_string())
            }
            let group_index = group_pk_map_vec[index]
                .get(&model_ending_array[2][0]).unwrap()
                .to_string().parse::<usize>().unwrap();
            let ending_group_id_map = ending_group_id_map_vec[group_index].clone();
            let _ending_pk_map = ending_pk_map_vec[group_index].clone();

            model_ending_array[1][0] = ending_group_id_map.get(&model_ending_array[1][0]).unwrap().to_string()
        }

        
        model_ending_array_vec.sort();
        model_ending_array_vec.dedup();
       
        for model_ending in model_ending_array_vec.into_iter() {
            let model_data_vec: Vec<String> = vec![
                model_ending[1][0].to_string(),
                model_ending[0][0].to_string()
            ];

            model_data_vec_vec.push(model_data_vec);
        } 
    }

    model_data_vec_vec.sort_by(|a, b| a[0].parse::<i64>().unwrap().cmp(&b[0].parse::<i64>().unwrap()));

    return model_data_vec_vec;
}


fn get_model_ending_id_map_vec_vec(model_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // map os <model, ending>
    let mut model_ending_id_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for model_data in model_json_data_vec {
        let mut model_ending_id_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_id: i64 = ending.parse::<i64>().unwrap();
            model_ending_id_map.insert(model.to_owned(), ending_id);

            while ending_id >= model_ending_id_map_vec.len().to_string().parse::<i64>().unwrap() {
                let mut initial_map: BTreeMap<String, i64> = BTreeMap::new();
                initial_map.insert(String::from("-"), model_ending_id_map_vec.len().to_string().parse::<i64>().unwrap());
                model_ending_id_map_vec.push(initial_map);
            }

            model_ending_id_map_vec[ending_id.to_string().parse::<usize>().unwrap()].append(&mut model_ending_id_map);
        }
    }

    return model_ending_id_map_vec;
}




fn get_model_pk_map_vec(model_json_data_vec: Vec<JsonData>) -> Vec<BTreeMap<String, i64>> {
    // map os <model, pk>
    let mut model_pk_map_vec: Vec<BTreeMap<String, i64>> = Vec::new(); 
    for model_data in model_json_data_vec {
        let mut model_pk_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::ModelField(ModelField {model, ending }) = &model_data.fields {
            let ending_id: i64 = ending.parse::<i64>().unwrap();
            model_pk_map.insert(model.to_owned(), model_data.pk);

            while ending_id >= model_pk_map_vec.len().to_string().parse::<i64>().unwrap() {
                let mut initial_map: BTreeMap<String, i64> = BTreeMap::new();
                initial_map.insert(String::from("-"), model_pk_map_vec.len().to_string().parse::<i64>().unwrap());
                model_pk_map_vec.push(initial_map);
            }

            model_pk_map_vec[ending_id.to_string().parse::<usize>().unwrap()].append(&mut model_pk_map);
        }
    }

    return model_pk_map_vec;
}

fn save_string_i64_map_vec(string_i64_map_vec: &Vec<BTreeMap<String, i64>>, file_path: &str) {
    let serialized_data: String = serde_json::to_string_pretty(&string_i64_map_vec).unwrap();
    fs::remove_file(file_path).unwrap();
    let mut file = open_file(file_path).unwrap();
    append_file(&mut file, &serialized_data);

    // println!("string_i64_map_vec: {:?}", string_i64_map_vec);
    // println!("file_path: {}", file_path);
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
