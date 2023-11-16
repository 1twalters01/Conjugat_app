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

// use std::{
//     collections::HashSet,
//     result,
// };




pub async fn run_model_module() {
    // get html vector for the models of each language saved
    let language_content: String = read_html_from_file("temp/json/languages/languages.json");
    let (_languages_data, languages) = read_language_data_from_json_data(language_content.as_str());

    // get vector of all the content
    let content_vec: Vec<String> = get_model_html_vec(languages);
    let document_vec: Vec<Html> = content_vec.into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data: Vec<Select> = get_main_data(&document_vec);

    // obtain the group data and group: language map, where 0: language, 1: group
    let groups_data_vec_vec: Vec<Vec<String>> = get_groups_data_vec_vec(&main_data);
    let groups_data: Vec<JsonData> = create_json_data_vec(groups_data_vec_vec, FieldOptions::GroupField);
    save_data_to_json_file(&groups_data, "temp/json/models/groups.json");
    let groups_vec_vec_map: Vec<Vec<BTreeMap<String, i64>>> = get_groups_vec_vec_map(groups_data);

    // obtain the ending data and ending: group map, where 0: group, 1: ending
    let endings_data_vec_vec: Vec<Vec<String>> = get_endings_data_vec_vec(&main_data, &groups_vec_vec_map);
    let endings_data: Vec<JsonData> = create_json_data_vec(endings_data_vec_vec, FieldOptions::EndingField);
    save_data_to_json_file(&endings_data, "temp/json/models/endings.json");
    let endings_vec_vec_map: Vec<Vec<BTreeMap<String, i64>>> = get_endings_vec_vec_map(endings_data);

    // obtain the model data and model: ending map, where 0: ending, 1: model
    // let models_data_vec_vec: Vec<Vec<String>> = get_models_data_vec_vec(&document_vec, &endings_data);
    // let models_data: Vec<JsonData> = create_json_data_vec(models_data_vec_vec, FieldOptions::ModelField);
    // let models_vec_vec_map: Vec<Vec<BTreeMap<String, String>>> = get_models_vec_vec_map(models_data);


    // let endings_file_path: &str = "temp/json/models/endings.json";
    // let models_file_path: &str = "temp/json/models/models.json";
    // save_data_to_json_file(&models_data, models_file_path);

    // save_data_to_postgres(&groups_data, &endings_data, &models_data).await;
}


fn read_language_data_from_json_data(language_content: &str) -> (Vec<JsonData>, Vec<String>) {
    // let language_file_path: &str = "temp/json/models/groups.json";
    // language_content = read_html_from_file(language_file_path);
    let languages_data: Vec<JsonData> = serde_json::from_str(language_content).unwrap();
    
    let mut languages: Vec<String> = Vec::new();
    for language_data in &languages_data {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            languages.push(language.clone());
        }
    }
    return (languages_data, languages);
}


fn get_model_html_vec(languages: Vec<String>) -> Vec<String> {
    let mut content_vec: Vec<String> = Vec::new();
    
    for language in languages {
        let file_path: String = String::from("temp/html/models/") + language.to_lowercase().as_str() + ".txt";
        println!("{}", file_path);
        let content: String = read_html_from_file(file_path.as_str());
        content_vec.push(content);
    }

    return content_vec;
}


fn get_main_data(document_vec: &Vec<Html>) -> Vec<Select> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    // let mut main_data: Vec<Html> = Vec::new();

    let main_data = document_vec.iter().enumerate()
    .map(|(index, document)| document.select(&main_selector)).collect::<Vec<_>>();
    
    return main_data;
}


fn get_groups_data_vec_vec(main_data_vec: &Vec<Select>) -> Vec<Vec<String>> {
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut groups_data_vec_vec: Vec<Vec<String>> = Vec::new();
   
    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut groups = main_data.into_iter()
            .map(|data| data.select(&group_selector).next().unwrap().text().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .iter().filter(|testvec| testvec.len() > 0)
            .map(|testvec| testvec[0]).collect::<Vec<_>>();

        groups.sort();
        groups.dedup();

        for group in groups {
            let group_vec: Vec<String> = vec![index.to_string(), group.to_string()];
            groups_data_vec_vec.push(group_vec);
        }
    }

    return groups_data_vec_vec;
}


use std::collections::BTreeMap;
fn get_groups_vec_vec_map(groups_data: Vec<JsonData>) -> Vec<Vec<BTreeMap<String, i64>>> {
    // Outer vec by language
    // map os <group, pk>
    let mut group_vec_vec_map: Vec<Vec<BTreeMap<String, i64>>> = Vec::new(); 
    for group_data in groups_data {
        let mut group_map: BTreeMap<String, i64> = BTreeMap::new();
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_map.insert(group.to_owned(), language_id);
            if language_id > group_vec_vec_map.len().to_string().parse::<i64>().unwrap() {
                group_vec_vec_map.push(Vec::from([group_map]));
            } else {
                group_vec_vec_map[language_id.to_string().parse::<usize>().unwrap()].push(group_map);
            }
        }
    }

    return group_vec_vec_map;
}


fn get_endings_data_vec_vec(document_vec: &Vec<Html>) -> Vec<Vec<String>> {
    let all_selector = scraper::Selector::parse("div.model-row").unwrap();
    let ending_selector = scraper::Selector::parse("").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut endings_data_vec_vec: Vec<Vec<String>> = Vec::new();
    let mut ending_count: i64 = 0;

    for (index, document) in document_vec.into_iter().enumerate() {
        let mut endings_groups_vec = document.select(&all_selector).into_iter()
            .map(|all_scraped| [all_scraped.select(&ending_selector).next.unwrap().text.collect::<Vec<_>>(),
                                all_scraped.select(group_selector).next.unwrap().text.collect::<Vec<_>>()]) // use map to turn word into int
            .collect::<Vec<_>>()
            .iter().filter() // Make all empty groups be equal to "-"
            .map(|testvecvec| testvecvec.map(|testvec| testvec[0]).collect::<Vec<_>>()).collect::<Vec<_>>();

        endings_groups_vec.sort();
        endings_groups_vec.dedup();
    }

    return ending_data_vec_vec;
}

    
fn get_models_data_vec_vec(document_vec: &Vec<Html>) -> Vec<Vec<String>> {
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

    return models_vec_vec
}




// fn get_models_data_vec_vec(document_vec: &Vec<Html>, groups_data: &Vec<JsonData>) -> Vec<Vec<String>> {
//     let all_selector = scraper::Selector::parse("div.model-row").unwrap();
//     // let model_selector = scraper::Selector::parse("a[class=model-title-verb]").unwrap();
//     let mut models_data_vec_vec: Vec<Vec<String>> = Vec::new();
//     let mut ending_model_data_vec_vec: Vec<Vec<String>> = Vec::new();
//
//     for document in document_vec {
//         let mut all: Vec<Vec<String>> = document.select(&all_selector).map(|el| el.text().map(|var| var.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
//
//         all.sort();
//
//         // This is defo wrong G
//         for (index, item) in all.into_iter().enumerate() {
//             let ending_vec: Vec<String> = Vec::from([groups_data[index].pk.to_string(), item[1].clone(), item[2].clone()]);
//             ending_model_data_vec_vec.push(ending_vec);
//         }
//
//         ending_model_data_vec_vec.sort();
//
//         for (index, ending_model_data_vec) in ending_model_data_vec_vec.clone().into_iter().enumerate() {
//             let model_vec: Vec<String> = Vec::from([index.to_string(), ending_model_data_vec[index].clone()]);
//             models_data_vec_vec.push(model_vec);
//         }
//
//         models_data_vec_vec.sort();
//    } 
//
//     return models_data_vec_vec;
// }



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
