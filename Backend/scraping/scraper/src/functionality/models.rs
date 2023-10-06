// Todo

// atomic counter for auto increment
static PK_COUNTER: AtomicI64 = AtomicI64::new(1);


pub async fn run_model_module() {



    let groups_data: Vec<JsonData> = create_groups_vec();
    let endings_data: Vec<JsonData> = create_endings_vec();
    let model_data: Vec<JsonData> = create_model_vec();

    let groups_file_path: String = "temp/json/models/groups.json".to_string();
    let endings_file_path: String = "temp/json/models/endings.json".to_string();
    let models_file_path: String = "temp/json/models/models.json".to_string();
    save_data_to_json_file(&groups_data, groups_file_path);
    save_data_to_json_file(&endings_data, endings_file_path);
    save_data_to_json_file(&models_data, models_file_path);
    
    save_to_postgres(&groups_data, &endings_data, &models_data).await;
}




fn create_groups_vec() -> Vec<JsonData {

}

fn create_groups_vec() -> Vec<JsonData {

}

fn create_groups_vec() -> Vec<JsonData {

}



async fn save_data_to_postgres(groups_data: &Vec<JsonData>, endings_data: &Vec<JsonData>, models_data: &Vec<JsonData>) {
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
