use crate::{
    data_types::{
        field::{Field, FieldOptions},
        field_options::{EndingField, GroupField, LanguageField, ModelField},
        json_data::{create_json_data_vec_from_vec_vec_string, JsonData},
    },
    helper_functions::{
        // postgres_functions::save_data_to_postgres,
        read_functions::read_file_to_string,
        save_functions::{save_json_data_vec_to_file, save_map_vec_to_file},
    },
};

use scraper::Html;

use std::collections::BTreeMap;

pub async fn run_model_module() {
    // Read languages data from file
    let language_content: String = read_file_to_string("temp/json/languages/languages.json");
    let language_vec = read_language_vec_from_language_json_data(language_content.as_str());
    let language_map_content: String =
        read_file_to_string("temp/json/languages/btreemaps/languages.json");
    let language_pk_map: BTreeMap<String, i64> =
        serde_json::from_str(&language_map_content).unwrap();

    // Read html for each language's model page on Reverso (saved for convenience)
    let content_vec: Vec<String> = get_model_content_vec(&language_vec);

    // Create group data and group:language map
    let group_data_vec_vec: Vec<Vec<String>> =
        get_group_data_vec_vec(&content_vec, &language_vec, language_pk_map);
    let group_json_data_vec: Vec<JsonData> =
        create_json_data_vec_from_vec_vec_string(&group_data_vec_vec, FieldOptions::GroupField);
    save_json_data_vec_to_file(&group_json_data_vec, "temp/json/models/groups.json");
    let group_language_id_map_vec: Vec<BTreeMap<String, i64>> =
        get_group_language_id_map_vec(&group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &group_language_id_map_vec,
        "temp/json/models/btreemaps/group_language_id.json",
    );
    let language_id_group_map_vec: Vec<BTreeMap<i64, String>> =
        get_language_id_group_map_vec(&group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &language_id_group_map_vec,
        "temp/json/models/btreemaps/language_id_group.json",
    );
    let group_pk_map_vec: Vec<BTreeMap<String, i64>> =
        get_group_pk_map_vec(&group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &group_pk_map_vec,
        "temp/json/models/btreemaps/group_pk.json",
    );
    let pk_group_map_vec: Vec<BTreeMap<i64, String>> =
        get_pk_group_map_vec(&group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &pk_group_map_vec,
        "temp/json/models/btreemaps/pk_group.json",
    );

    // Create ending data and ending:group map
    let ending_data_vec_vec: Vec<Vec<String>> =
        get_ending_data_vec(&content_vec, &group_pk_map_vec);
    let ending_json_data_vec: Vec<JsonData> =
        create_json_data_vec_from_vec_vec_string(&ending_data_vec_vec, FieldOptions::EndingField);
    save_json_data_vec_to_file(&ending_json_data_vec, "temp/json/models/endings.json");
    let ending_group_id_map_vec: Vec<BTreeMap<String, i64>> =
        get_ending_group_id_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &ending_group_id_map_vec,
        "temp/json/models/btreemaps/ending_group_id.json",
    );
    let group_id_ending_map_vec: Vec<BTreeMap<i64, String>> =
        get_group_id_ending_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &group_id_ending_map_vec,
        "temp/json/models/btreemaps/group_id_ending.json",
    );
    let ending_language_id_map_vec: Vec<BTreeMap<String, i64>> =
        get_ending_language_id_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &ending_language_id_map_vec,
        "temp/json/models/btreemaps/ending_language_id.json",
    );
    let language_id_ending_map_vec: Vec<BTreeMap<i64, String>> =
        get_language_id_ending_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &language_id_ending_map_vec,
        "temp/json/models/btreemaps/language_id_ending.json",
    );
    let ending_pk_map_vec: Vec<BTreeMap<String, i64>> =
        get_ending_pk_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &ending_pk_map_vec,
        "temp/json/models/btreemaps/ending_pk.json",
    );
    let pk_ending_map_vec: Vec<BTreeMap<i64, String>> =
        get_pk_ending_map_vec(&ending_json_data_vec, &group_json_data_vec, &language_vec);
    save_map_vec_to_file(
        &pk_ending_map_vec,
        "temp/json/models/btreemaps/pk_ending.json",
    );

    // Create model data and model:ending map
    let model_data_vec_vec: Vec<Vec<String>> =
        get_model_data_vec_vec(&content_vec, &ending_pk_map_vec);
    let model_json_data_vec: Vec<JsonData> =
        create_json_data_vec_from_vec_vec_string(&model_data_vec_vec, FieldOptions::ModelField);
    save_json_data_vec_to_file(&model_json_data_vec, "temp/json/models/models.json");
    let model_ending_id_map_vec: Vec<BTreeMap<String, i64>> = get_model_ending_id_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &model_ending_id_map_vec,
        "temp/json/models/btreemaps/model_ending.json",
    );
    let ending_id_model_map_vec: Vec<BTreeMap<i64, String>> = get_ending_id_model_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &ending_id_model_map_vec,
        "temp/json/models/btreemaps/ending_id_model.json",
    );
    let model_language_id_map_vec: Vec<BTreeMap<String, i64>> = get_model_language_id_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &model_language_id_map_vec,
        "temp/json/models/btreemaps/model_language_id.json",
    );
    let language_id_model_map_vec: Vec<BTreeMap<i64, String>> = get_language_id_model_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &language_id_model_map_vec,
        "temp/json/models/btreemaps/language_id_model.json",
    );
    let model_pk_map_vec: Vec<BTreeMap<String, i64>> = get_model_pk_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &model_pk_map_vec,
        "temp/json/models/btreemaps/model_pk.json",
    );
    let pk_model_map_vec: Vec<BTreeMap<i64, String>> = get_pk_model_map_vec(
        &model_json_data_vec,
        &ending_json_data_vec,
        &group_json_data_vec,
        &language_vec,
    );
    save_map_vec_to_file(
        &pk_model_map_vec,
        "temp/json/models/btreemaps/pk_model.json",
    );

    // save_data_to_postgres(&group_json_data_vec).await;
    // save_data_to_postgres(&ending_json_data_vec).await;
    // save_data_to_postgres(&model_json_data_vec).await;
}

fn read_language_vec_from_language_json_data(language_content: &str) -> Vec<String> {
    let language_data_vec: Vec<JsonData> = serde_json::from_str(language_content).unwrap();
    let mut language_vec: Vec<String> = Vec::new();

    for language_data in &language_data_vec {
        if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
            language_vec.push(language.clone());
        }
    }

    return language_vec;
}

fn get_model_content_vec(language_vec: &Vec<String>) -> Vec<String> {
    let mut content_vec: Vec<String> = Vec::new();

    for language in language_vec {
        let file_path: String =
            String::from("temp/html/models/") + language.to_lowercase().as_str() + ".txt";
        let content: String = read_file_to_string(file_path.as_str());
        content_vec.push(content);
    }

    return content_vec;
}

fn get_group_data_vec_vec(
    content_vec: &Vec<String>,
    language_vec: &Vec<String>,
    language_pk_map: BTreeMap<String, i64>,
) -> Vec<Vec<String>> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec
        .into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec
        .iter()
        .map(|document| document.select(&main_selector))
        .collect::<Vec<_>>();

    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut group_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let language: String = language_pk_map
            .get(&language_vec[index])
            .unwrap()
            .to_string();

        let mut group_vec = main_data
            .into_iter()
            .map(|data| {
                data.select(&group_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .iter()
            .filter(|testvec| testvec.len() > 0)
            .map(|testvec| testvec[0])
            .collect::<Vec<_>>();

        group_vec.sort();
        group_vec.dedup();

        group_data_vec_vec.push(vec![language.clone(), "-".to_string()]);
        for group in group_vec {
            let group_vec: Vec<String> = vec![language.clone(), group.to_string()];
            group_data_vec_vec.push(group_vec);
        }
    }

    return group_data_vec_vec;
}

fn get_group_language_id_map_vec(
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut group_language_id_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for group_data in group_json_data_vec {
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_language_id_map_vec[language_id as usize - 1].insert(group.clone(), language_id);
        }
    }

    return group_language_id_map_vec;
}

fn get_language_id_group_map_vec(
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut language_id_group_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for group_data in group_json_data_vec {
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            language_id_group_map_vec[language_id as usize - 1].insert(language_id, group.clone());
        }
    }

    return language_id_group_map_vec;
}

fn get_group_pk_map_vec(
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut group_pk_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for group_data in group_json_data_vec {
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            group_pk_map_vec[language_id as usize - 1].insert(group.clone(), group_data.pk);
        }
    }

    return group_pk_map_vec;
}

fn get_pk_group_map_vec(
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut pk_group_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for group_data in group_json_data_vec {
        if let Field::GroupField(GroupField { group, language }) = &group_data.fields {
            let language_id: i64 = language.parse::<i64>().unwrap();
            pk_group_map_vec[language_id as usize - 1].insert(group_data.pk, group.clone());
        }
    }

    return pk_group_map_vec;
}

fn get_ending_data_vec(
    content_vec: &Vec<String>,
    group_pk_map_vec: &Vec<BTreeMap<String, i64>>,
) -> Vec<Vec<String>> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec
        .into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec
        .iter()
        .map(|document| document.select(&main_selector))
        .collect::<Vec<_>>();

    let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut ending_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut ending_group_array_vec: Vec<Vec<Vec<String>>> = main_data
            .into_iter()
            .map(|data| {
                vec![
                    data.select(&ending_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|data| data.trim().to_string())
                        .collect::<Vec<String>>(),
                    data.select(&group_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|data| data.trim().to_string())
                        .collect::<Vec<String>>(),
                ]
            })
            .collect::<Vec<_>>();

        let blank_ending_group_array: Vec<Vec<String>> =
            vec![Vec::from(["-".to_string()]), Vec::from(["-".to_string()])];
        ending_group_array_vec.push(blank_ending_group_array);

        // println!("group_pk_map_vec: {:?}", group_pk_map_vec);
        for ending_group_array in ending_group_array_vec.iter_mut() {
            if ending_group_array[1].len() == 0 {
                ending_group_array[1].push("-".to_string());
            }
            ending_group_array[1][0] = group_pk_map_vec[index]
                .get(&ending_group_array[1][0])
                .unwrap()
                .to_string()
        }

        ending_group_array_vec.sort();
        ending_group_array_vec.dedup();

        for ending_group in ending_group_array_vec.into_iter() {
            let ending_data_vec: Vec<String> = vec![
                ending_group[1][0].to_string(),
                ending_group[0][0].to_string(),
            ];

            ending_data_vec_vec.push(ending_data_vec);
        }
    }

    ending_data_vec_vec.sort_by(|a, b| {
        a[0].parse::<i64>()
            .unwrap()
            .cmp(&b[0].parse::<i64>().unwrap())
    });

    // panic!("pause");
    return ending_data_vec_vec;
}

fn get_ending_group_id_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut ending_group_id_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            ending_group_id_map_vec[language_id as usize - 1].insert(ending.clone(), group_id);
        }
    }

    return ending_group_id_map_vec;
}

fn get_group_id_ending_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut group_id_ending_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            group_id_ending_map_vec[language_id as usize - 1].insert(group_id, ending.clone());
        }
    }

    return group_id_ending_map_vec;
}

fn get_ending_language_id_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut ending_language_id_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            ending_language_id_map_vec[language_id as usize - 1]
                .insert(ending.clone(), language_id);
        }
    }

    return ending_language_id_map_vec;
}

fn get_language_id_ending_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut language_id_ending_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            language_id_ending_map_vec[language_id as usize - 1]
                .insert(language_id, ending.clone());
        }
    }

    return language_id_ending_map_vec;
}

fn get_ending_pk_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut ending_pk_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            ending_pk_map_vec[language_id as usize - 1].insert(ending.to_owned(), ending_data.pk);
        }
    }

    return ending_pk_map_vec;
}

fn get_pk_ending_map_vec(
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut pk_ending_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for ending_data in ending_json_data_vec {
        if let Field::EndingField(EndingField { ending, group }) = &ending_data.fields {
            let group_id: i64 = group.parse::<i64>().unwrap();

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_id as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            pk_ending_map_vec[language_id as usize - 1].insert(ending_data.pk, ending.to_owned());
        }
    }

    return pk_ending_map_vec;
}

fn get_model_data_vec_vec(
    content_vec: &Vec<String>,
    ending_pk_map_vec: &Vec<BTreeMap<String, i64>>,
) -> Vec<Vec<String>> {
    let main_selector = scraper::Selector::parse("div.model-row").unwrap();
    let document_vec: Vec<Html> = content_vec
        .into_iter()
        .map(|extract| scraper::Html::parse_document(&extract))
        .collect::<Vec<Html>>();
    let main_data_vec = document_vec
        .iter()
        .map(|document| document.select(&main_selector))
        .collect::<Vec<_>>();

    let model_selector = scraper::Selector::parse("a[class=model-title-verb]").unwrap();
    let ending_selector = scraper::Selector::parse("p[class=ending]").unwrap();
    let group_selector = scraper::Selector::parse("p[class=group]").unwrap();
    let mut model_data_vec_vec: Vec<Vec<String>> = Vec::new();

    for (index, main_data) in main_data_vec.into_iter().enumerate() {
        let mut model_ending_array_vec: Vec<Vec<Vec<String>>> = main_data
            .into_iter()
            .map(|data| {
                vec![
                    data.select(&model_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|data| data.trim().to_string())
                        .collect::<Vec<String>>(),
                    data.select(&ending_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|data| data.trim().to_string())
                        .collect::<Vec<String>>(),
                    data.select(&group_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|data| data.trim().to_string())
                        .collect::<Vec<String>>(),
                ]
            })
            .collect::<Vec<_>>();

        let blank_model_ending_array: Vec<Vec<String>> = vec![
            Vec::from(["-".to_string()]),
            Vec::from(["-".to_string()]),
            Vec::from(["-".to_string()]),
        ];
        model_ending_array_vec.push(blank_model_ending_array);

        for model_ending_array in model_ending_array_vec.iter_mut() {
            if model_ending_array[1].len() == 0 {
                model_ending_array[1].push("-".to_string())
            }
            if model_ending_array[2].len() == 0 {
                model_ending_array[2].push("-".to_string())
            }

            let ending_pk_map = ending_pk_map_vec[index].clone();

            model_ending_array[1][0] = ending_pk_map
                .get(&model_ending_array[1][0])
                .unwrap()
                .to_string()
        }

        model_ending_array_vec.sort();
        model_ending_array_vec.dedup();

        for model_ending in model_ending_array_vec.into_iter() {
            let model_data_vec: Vec<String> = vec![
                model_ending[1][0].to_string(),
                model_ending[0][0].to_string(),
            ];

            model_data_vec_vec.push(model_data_vec);
        }
    }

    model_data_vec_vec.sort_by(|a, b| {
        a[0].parse::<i64>()
            .unwrap()
            .cmp(&b[0].parse::<i64>().unwrap())
    });

    return model_data_vec_vec;
}

fn get_model_ending_id_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut model_ending_id_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }
            model_ending_id_map_vec[language_id as usize - 1].insert(model.clone(), ending_pk);
        }
    }

    return model_ending_id_map_vec;
}

fn get_ending_id_model_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut ending_id_model_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }
            ending_id_model_map_vec[language_id as usize - 1].insert(ending_pk, model.clone());
        }
    }

    return ending_id_model_map_vec;
}

fn get_model_language_id_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut model_ending_id_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            model_ending_id_map_vec[language_id as usize - 1].insert(model.clone(), language_id);
        }
    }

    return model_ending_id_map_vec;
}

fn get_language_id_model_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut ending_id_model_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }

            ending_id_model_map_vec[language_id as usize - 1].insert(language_id, model.clone());
        }
    }

    return ending_id_model_map_vec;
}

fn get_model_pk_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<String, i64>> {
    let mut model_pk_map_vec: Vec<BTreeMap<String, i64>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }
            model_pk_map_vec[language_id as usize - 1].insert(model.clone(), model_data.pk);
        }
    }

    return model_pk_map_vec;
}

fn get_pk_model_map_vec(
    model_json_data_vec: &Vec<JsonData>,
    ending_json_data_vec: &Vec<JsonData>,
    group_json_data_vec: &Vec<JsonData>,
    language_vec: &Vec<String>,
) -> Vec<BTreeMap<i64, String>> {
    let mut pk_model_map_vec: Vec<BTreeMap<i64, String>> =
        language_vec.into_iter().map(|_| BTreeMap::new()).collect();

    for model_data in model_json_data_vec {
        if let Field::ModelField(ModelField { model, ending }) = &model_data.fields {
            let ending_pk: i64 = ending.parse::<i64>().unwrap();

            let mut group_pk: i64 = 0;
            if let Field::EndingField(EndingField { ending: _, group }) =
                ending_json_data_vec[ending_pk as usize - 1].clone().fields
            {
                group_pk = group.parse::<i64>().unwrap();
            }

            let mut language_id: i64 = 0;
            if let Field::GroupField(GroupField { group: _, language }) =
                group_json_data_vec[group_pk as usize - 1].clone().fields
            {
                language_id = language.parse::<i64>().unwrap();
            }
            pk_model_map_vec[language_id as usize - 1].insert(model_data.pk, model.clone());
        }
    }

    return pk_model_map_vec;
}
