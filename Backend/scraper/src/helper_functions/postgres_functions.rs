use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    io::Error,
    result::Result,
};

use crate::data_types::{
    json_data::JsonData,
    field::{
        Field,
        FieldOptions,
    },
    field_options::{
        LanguageField,
        GroupField,
        ModelField,
        EndingField,
        BaseField,
        TenseField,
        TenseSubfield,
        SubjectField,
        AuxiliaryField,
        ConjugateField,
        ConjugationField,
        SentenceField,
    },
};



pub async fn create_pool_connection() -> Pool<Postgres> {
    let pgusername: String = env::var("PG_USERNAME").unwrap();
    let pgpassword: String = env::var("PG_PASSWORD").unwrap();
    let pgdbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = String::from("postgres://") + pgusername.as_str() + ":"
        + pgpassword.as_str() + "@localhost:5432/" + pgdbname.as_str();

    // Create connection pool 
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await.unwrap();
  
    return pool
}

async fn save_data_to_postgres(json_data_vec: &Vec<JsonData>) -> Result<(), io::Error> {
    let pool: Pool<Postgres> = create_pool_connection().await;

    for json_data in json_data_vec {
        let field_result = match &json_data.fields {
            Field::LanguageField(LanguageField{language}) => save_language_field_to_postgres(pool, language),
            Field::GroupField(GroupField{language, group}) => save_group_field_to_postgres(pool, language, group),
            Field::EndingField(EndingField{group, ending}) => save_ending_field_to_postgres(pool, group, ending),
            Field::ModelField(ModelField{ending, model}) => save_model_field_to_postgres(pool, ending, model),
            Field::BaseField(BaseField{rank, language, base}) => save_base_field_to_postgres(pool, rank, language, base),
            Field::TenseField(TenseField{language, tense}) => save_tense_field_to_postgres(pool, language, tense),
            Field::SubjectField(SubjectField{language, subject}) => save_subject_field_to_postgres(pool, language, subject),
            Field::AuxiliaryField(AuxiliaryField{language, auxiliary}) => save_auxiliary_field_to_postgres(pool, language, auxilary),
            Field::ConjugateField(ConjugateField{base, conjugate, model}) => save_conjugate_field_to_postgres(pool, base, conjugate, model),
            Field::ConjugationField(ConjugationField{rank, tense, subject, auxiliary, conjugate}) => save_conjugation_field_to_postgres(pool, rank, tense, subject, auxiliary, conjugate),
            Field::SentenceField(SentenceField{rank, conjugation, sentence, char_length, char_start}) => save_sentence_field_to_postgres(pool, rank, conjugation, sentence, char_length, char_start
        };

        match field_result {
            Ok(_) => {},
            Err(err) => return Err(err),
        };
    }

    return Ok(());
}

async fn save_language_field_to_postgres(pool: Postgres<Postgres>, language: &str) -> Result<(), Error> {
    if let Field::LanguageField(LanguageField { language }) = &language_data.fields {
        let insert_query = sqlx::query("INSERT INTO verbs_language (id, language) VALUES ($1, $2)")
            .bind(language_data.pk)
            .bind(language)
            .execute(&pool).await;

        match insert_query {
            Ok(res) => {res},
            Err(_) => {
                let update_query = sqlx::query("UPDATE verbs_lanauge SET lanague=($1), WHERE id=($2)")
                    .bind(language)
                    .bind(language_data.pk)
                    .execute(&pool).await;

                let update_result = match update_query {
                    Ok(res) => res,
                    Err(err) => panic!("Error: {:?}", err),
                };
                update_result
            }
        };
    }
}

async fn save_group_field_to_postgres(pool: Pool<Postgres>, language: &str, group: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
        .bind(group_data.pk)
        .bind(language)
        .bind(group)
        .execute(&pool)
        .await;

    match insert_query {
        Ok(res) => res,
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
                .bind(language)
                .bind(group)
                .bind(group_data.pk)
                .execute(&pool)
                .await;

            let rewrite_result = match rewrite_query {
                Ok(res) => res,
                Err(err) => panic!("Error: {:?}", err),
            };
            rewrite_result
        },
    };

}

async fn save_ending_field_to_postgres(pool: Pool<Postgres>, group: &str, ending: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
        .bind(ending_data.pk)
        .bind(group)
        .bind(ending)
        .execute(&pool).await;

    match insert_query {
        Ok(res) => res,
        Err(_) => {
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
    }
}


async fn save_model_field_to_postgres(pool: Pool<Postgres>, ending: &str, model: &str) -> Result<(), Error> {
    if let Field::ModelField(ModelField { ending, model }) = &model_data.fields {
        let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
            .bind(model_data.pk)
            .bind(ending)
            .bind(model)
            .execute(&pool).await;

        match insert_query {
            Ok(res) => res,
            Err(_) => {
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
    }
}


async fn save_base_field_to_postgres() -> Result<(), Error> {}
