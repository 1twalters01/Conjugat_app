use serde::{Serialize, Deserialize};
use crate::data_types::field::{
    Field,
    FieldOptions,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonData {
    pub model: String,
    pub pk: i64,
    pub fields: Field,
}


impl JsonData {
    pub fn default(field_type: &FieldOptions) -> JsonData {
        match field_type {
            FieldOptions::LanguageField => {
                return JsonData {
                    model: String::from("verbs.languages"),
                    pk: 0,
                    fields: Field::default(FieldOptions::LanguageField),
                }
            },
            
            FieldOptions::GroupField => {
                return JsonData {
                    model: String::from("verbs.groups"),
                    pk: 0,
                    fields: Field::default(FieldOptions::GroupField),
                }
            },

            FieldOptions::EndingField => {
                return JsonData {
                    model: String::from("verbs.endings"),
                    pk: 0,
                    fields: Field::default(FieldOptions::EndingField),
                }
            },
 
            FieldOptions::ModelField => {
                return JsonData {
                    model: String::from("verbs.models"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ModelField),
                }
            },

            FieldOptions::BaseField => {
                return JsonData {
                    model: String::from("verbs.bases"),
                    pk: 0,
                    fields: Field::default(FieldOptions::BaseField),
                }
            },

            FieldOptions::TenseField => {
                return JsonData {
                    model: String::from("verbs.tenses"),
                    pk: 0,
                    fields: Field::default(FieldOptions::TenseField),
                }
            },
 
            FieldOptions::SubjectField => {
                return JsonData {
                    model: String::from("verbs.subjects"),
                    pk: 0,
                    fields: Field::default(FieldOptions::SubjectField),
                }
            },
 
            FieldOptions::AuxiliaryField => {
                return JsonData {
                    model: String::from("verbs.auxiliaries"),
                    pk: 0,
                    fields: Field::default(FieldOptions::AuxiliaryField),
                }
            },
 
            FieldOptions::ConjugateField => {
                return JsonData {
                    model: String::from("verbs.conjugates"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugateField),
                }
            },
            
            FieldOptions::ConjugationField => {
                return JsonData {
                    model: String::from("verbs.conjugations"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugationField),
                }
            },
            
            FieldOptions::SentenceField => {
                return JsonData {
                    model: String::from("verbs.sentences"),
                    pk: 0,
                    fields: Field::default(FieldOptions::SentenceField),
                }
            },
        }
    }
}


