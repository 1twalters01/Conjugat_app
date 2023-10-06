// Todo

// atomic counter for auto increment
static PK_COUNTER: AtomicI64 = AtomicI64::new(1);


pub async fn run_model_module() {
    let (language_hash, languages) = read_language_json();
    // println!("language hash {:?}\n\nlanguages {:?}\n\n", language_hash, languages);
    
    // let (all, groups, endings, models) = scrape_html(&languages);
    let (all) = scrape_html(&languages);
    let (groups, endings, models) = split_vec(&all);
    // println!("all\n{:?}\n\nmodels\n{:?}\n\ngroups\n{:?}\n\nendings{:?}\n\n", all, models, groups, endings);


    let (groups_dict, endings_groups_dict, models_endings_dict) = generate_languages_hashmaps(&all);
    // println!("groups_dict\n{:?}\n\nendings_groups_dict\n{:?}\n\nmodels_endings_dict\n{:?}\n\n", groups_dict, endings_groups_dict, models_endings_dict);


    let (groups_data, endings_data, models_data) = generate_vectors(&all, &groups_dict, &endings_groups_dict, &models_endings_dict);
    // println!("groups_data\n{:?}\n\nendings_data\n{:?}\n\nmodels_data\n{:?}\n\n", groups_data, endings_data, models_data);;


    generate_json_files(&groups_data, &endings_data, &models_data);
    save_to_postgres(&groups_data, &endings_data, &models_data).await;
}









async fn save_data_to_postgres(languages_data: &Vec<JsonData>) {
    pool = create_pool_connection()

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
