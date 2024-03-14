use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    io::Error,
    result::Result,
};

use crate::data_types::{
    json_data::JsonData,
    field::Field,
    field_options::{
        LanguageField,
        GroupField,
        ModelField,
        EndingField,
        BaseField,
        MajorTenseField,
        MinorTenseField,
        TenseField,
        TenseSubfield,
        ParticleField,
        SubjectField,
        AuxiliaryField,
        ConjugateField,
        ConjugationField,
        SentenceField,
    },
};



async fn create_pool_connection() -> Pool<Postgres> {
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



pub async fn save_data_to_postgres(json_data_vec: &Vec<JsonData>) -> Result<(), Error> {
    let pool: Pool<Postgres> = create_pool_connection().await;

    for json_data in json_data_vec {
        let pk: i64 = json_data.pk;
        let field_result = match &json_data.fields {
            Field::LanguageField(LanguageField{language}) => save_language_field_to_postgres(&pool, pk, language).await,
            Field::GroupField(GroupField{language, group}) => save_group_field_to_postgres(&pool, pk, language, group).await,
            Field::EndingField(EndingField{group, ending}) => save_ending_field_to_postgres(&pool, pk, group, ending).await,
            Field::ModelField(ModelField{ending, model}) => save_model_field_to_postgres(&pool, pk, ending, model).await,
            Field::BaseField(BaseField{rank, language, base}) => save_base_field_to_postgres(&pool, pk, rank, language, base).await,
            Field::MajorTenseField(MajorTenseField{language, major_tense}) => save_major_tense_field_to_postgres(&pool, pk, language, major_tense).await,
            Field::MinorTenseField(MinorTenseField{language, minor_tense}) => save_minor_tense_field_to_postgres(&pool, pk, language, minor_tense).await,
            Field::TenseField(TenseField{rank, tense}) => save_tense_field_to_postgres(&pool, pk, rank, tense).await,
            Field::ParticleField(ParticleField{language, particle}) => save_particle_field_to_postgres(&pool, pk, language, particle).await,
            Field::SubjectField(SubjectField{rank, language, subject}) => save_subject_field_to_postgres(&pool, pk, rank, language, subject).await,
            Field::AuxiliaryField(AuxiliaryField{language, auxiliary}) => save_auxiliary_field_to_postgres(&pool, pk, language, auxiliary).await,
            Field::ConjugateField(ConjugateField{rank, base, conjugate, model}) => save_conjugate_field_to_postgres(&pool, pk, rank, base, model, conjugate).await,
            Field::ConjugationField(ConjugationField{rank, tense, particle, subject, auxiliary, conjugate}) => save_conjugation_field_to_postgres(&pool, pk, rank, tense, particle, subject, auxiliary, conjugate).await,
            Field::SentenceField(SentenceField{rank, conjugation, sentence, char_length, char_start}) => save_sentence_field_to_postgres(&pool, pk, rank, conjugation, sentence, char_length, char_start).await,
        };

        match field_result {
            Ok(_) => {},
            Err(err) => return Err(err),
        };
    }

    return Ok(());
}

async fn save_language_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_language (id, language) VALUES ($1, $2)")
        .bind(pk)
        .bind(language)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let update_query = sqlx::query("UPDATE verbs_langauge SET lanague=($1), WHERE id=($2)")
                .bind(language)
                .bind(pk)
                .execute(pool)
                .await;

            // let update_result = match update_query {
            //     Ok(res) => res,
            //     Err(err) => panic!("Error: {:?}", err),
            // };
            // update_result
            match update_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}

async fn save_group_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str, group: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_group (id, language, group) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(language)
        .bind(group)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_group SET language=($1), group=($2), WHERE id=($3)")
                .bind(language)
                .bind(group)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };

}

async fn save_ending_field_to_postgres(pool: &Pool<Postgres>, pk: i64, group: &str, ending: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_ending (id, group, ending) VALUES ($1, $2, $3")
        .bind(pk)
        .bind(group)
        .bind(ending)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_ending SET group=($1), ending=($2), WHERE id=($3)")
                .bind(group)
                .bind(ending)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        }
    };
}


async fn save_model_field_to_postgres(pool: &Pool<Postgres>, pk: i64, ending: &str, model: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_model (id, ending, model) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(ending)
        .bind(model)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_model SET ending=($1), model=($2) WHERE id=($3)")
                .bind(ending)
                .bind(model)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}


async fn save_base_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, language: &str, base: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_base (id, rank, language, base) VALUES ($1, $2, $3, $4)")
        .bind(pk)
        .bind(rank)
        .bind(language)
        .bind(base)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_base SET rank=($1), language=($2), base=($3) WHERE id=($4)")
                .bind(rank)
                .bind(language)
                .bind(base)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}

async fn save_major_tense_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str, tense_major: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_major_tense (id, language, tense_major) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(language)
        .bind(&tense_major)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_major_tense SET language=($1), tense_major=($2) WHERE id=($3)")
                .bind(language)
                .bind(&tense_major)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}

async fn save_minor_tense_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str, tense_minor: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_minor_tense (id, language, tense_minor) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(language)
        .bind(&tense_minor)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET language=($1), tense_minor=($2) WHERE id=($3)")
                .bind(language)
                .bind(&tense_minor)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}

async fn save_tense_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, tense: &TenseSubfield) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_tense (id, rank, tense_major, tense_minor) VALUES ($1, $2, $3, $4)")
        .bind(pk)
        .bind(rank)
        .bind(&tense.major)
        .bind(&tense.minor)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET rank=($1), tense_major=($2), tense_minor=($3) WHERE id=($4)")
                .bind(rank)
                .bind(&tense.major)
                .bind(&tense.minor)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}


async fn save_particle_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str, particle:&str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_particle (id, language, particle) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(language)
        .bind(particle)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET language=($1), particle=($2) WHERE id=($3)")
                .bind(language)
                .bind(particle)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}


async fn save_subject_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, language: &str, subject:&str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_subject (id, rank, language, subject) VALUES ($1, $2, $3, $4)")
        .bind(pk)
        .bind(rank)
        .bind(language)
        .bind(subject)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_subject SET rank=($1), language=($2), subject=($3) WHERE id=($4)")
                .bind(rank)
                .bind(language)
                .bind(subject)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}


async fn save_auxiliary_field_to_postgres(pool: &Pool<Postgres>, pk: i64, language: &str, auxiliary:&str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_auxiliary (id, language, auxiliary) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(language)
        .bind(auxiliary)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET language=($1), auxiliary=($2) WHERE id=($3)")
                .bind(language)
                .bind(auxiliary)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };
}


async fn save_conjugate_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, base: &str, model: &str, conjugate: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_auxiliary (id, rank, base, model, conjugate) VALUES ($1, $2, $3, $4, $5)")
        .bind(pk)
        .bind(rank)
        .bind(base)
        .bind(model)
        .bind(conjugate)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET rank=($1), base=($2), model=($3), conjugate=($4) WHERE id=($5)")
                .bind(rank)
                .bind(base)
                .bind(model)
                .bind(conjugate)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };

}


async fn save_conjugation_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, tense: &str, particle:&str, subject: &str, auxiliary: &str, conjugate: &str) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_auxiliary (id, rank, tense, particle, subject, auxiliary, conjugate) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(pk)
        .bind(rank)
        .bind(tense)
        .bind(particle)
        .bind(subject)
        .bind(auxiliary)
        .bind(conjugate)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET rank=($1), tense=($2), particle=($3), subject=($4), auxiliary=($5), conjugate=($6) WHERE id=($7)")
                .bind(rank)
                .bind(tense)
                .bind(particle)
                .bind(subject)
                .bind(auxiliary)
                .bind(conjugate)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };

}


async fn save_sentence_field_to_postgres(pool: &Pool<Postgres>, pk: i64, rank: &i64, conjugation: &str, sentence: &str, char_length: &i64, char_start: &i64) -> Result<(), Error> {
    let insert_query = sqlx::query("INSERT INTO verbs_auxiliary (id, language, auxiliary) VALUES ($1, $2, $3)")
        .bind(pk)
        .bind(rank)
        .bind(conjugation)
        .bind(sentence)
        .bind(char_length)
        .bind(char_start)
        .execute(pool)
        .await;

    match insert_query {
        Ok(_) => return Ok(()),
        Err(_) => {
            let rewrite_query = sqlx::query("UPDATE verbs_tense SET language=($1), auxiliary=($2) WHERE id=($3)")
                .bind(rank)
                .bind(conjugation)
                .bind(sentence)
                .bind(char_length)
                .bind(char_start)
                .bind(pk)
                .execute(pool)
                .await;

            match rewrite_query {
                Ok(_) => return Ok(()),
                Err(err) => panic!("Error: {:?}", err),
            };
        },
    };

}
 
