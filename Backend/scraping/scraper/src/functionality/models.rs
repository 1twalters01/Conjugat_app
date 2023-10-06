// Todo

// atomic counter for auto increment
static GROUP_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static ENDING_PK_COUNTER: AtomicI64 = AtomicI64::new(1);
static MODEL_PK_COUNTER: AtomicI64 = AtomicI64::new(1);



pub async fn run_model_module() {
    let groups: Vec<Vec<&str>> = Vec::new();
    let endings: Vec<Vec<&str>> = Vec::new();
    let models: Vec<Vec<&str>> = Vec::new();


    let groups_data: Vec<JsonData> = create_groups_vec(groups);
    let endings_data: Vec<JsonData> = create_endings_vec(endings);
    let model_data: Vec<JsonData> = create_model_vec(model);

    let groups_file_path: String = "temp/json/models/groups.json".to_string();
    let endings_file_path: String = "temp/json/models/endings.json".to_string();
    let models_file_path: String = "temp/json/models/models.json".to_string();
    save_data_to_json_file(&groups_data, groups_file_path);
    save_data_to_json_file(&endings_data, endings_file_path);
    save_data_to_json_file(&models_data, models_file_path);
    
    save_to_postgres(&groups_data, &endings_data, &models_data).await;
}




fn create_groups_vec(groups: Vec<Vec<&str>>) -> Vec<JsonData> {
    let mut groups_data: Vec<JsonData> = Vec::new();
    let pk_count: i64 = 1;

    for (index, group_vec) in groups.into_iter().enumerate() {
        for group in group_vec {
            let group_field = GroupField {
                language: (index+1).to_string(),
                group: group.to_string(),
            };
            let group_data = JsonData {
                //pk: GROUP_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                pk: pk_count,
                fields: Field::GroupField(group_field),
                ..JsonData::default(),
            };

            groups_data.push(group_data);
            pk_count = pk_count + 1;
        }
    }

    return groups_data;
}

fn create_endings_vec(endings: Vec<Vec<&str>>) -> Vec<JsonData> {
    let mut endings_data: Vec<JsonData> = Vec::new();
    let pk_count: i64 = 1;

    for (index, ending_vec) in endings.into_iter().enumerate() {
        for ending in ending_vec {
            let ending_field = EndingField {
                group: (index+1).to_string(),
                ending: ending.to_string(),
            };
            let ending_data = JsonData {
                //pk: ENDING_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                pk: pk_count,
                fields: Field::EndingField(ending_field),
                ..JsonData::default(),
            };

            endings_data.push(ending_data);
            pk_count = pk_count + 1;
        }
    }

    return endings_data;
}

fn create_models_vec(models: Vec<Vec<&str>>) -> Vec<JsonData> {
    let mut models_data: Vec<JsonData> = Vec::new();
    let pk_count: i64 = 1;

    for (index, model_vec) in models.into_iter().enumerate() {
        for model in model_vec {
            let model_field = ModelField {
                group: (index+1).to_string(),
                model: model.to_string(),
            };
            let model_data = JsonData {
                //pk: MODEL_PK_COUNTER.fetch_add(1, Ordering::SeqCst),
                pk: pk_count,
                fields: Field::ModelField(model_field),
                ..JsonData::default(),
            };

            models_data.push(model_data);
            pk_count = pk_count + 1;
        }
    }
    
    return models_data;
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
