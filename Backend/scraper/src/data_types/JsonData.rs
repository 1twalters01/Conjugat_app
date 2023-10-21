use serde::{Serialize, Deserialize};
use crate::data_types::Field::{
    Field,
    FieldOptions,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonData<'a> {
    model: &'a str,
    pub pk: i64,
    pub fields: Field<'a>,
}


impl<'a> JsonData<'a> {
    pub fn default(field_type: &FieldOptions) -> JsonData {
        match field_type {
            FieldOptions::LanguageField => {
                return JsonData {
                    model: "verbs.languages",
                    pk: 0,
                    fields: Field::default(FieldOptions::LanguageField),
                }
            },
            
            FieldOptions::GroupField => {
                return JsonData {
                    model: "verbs.groups",
                    pk: 0,
                    fields: Field::default(FieldOptions::GroupField),
                }
            },

            FieldOptions::EndingField => {
                return JsonData {
                    model: "verbs.endings",
                    pk: 0,
                    fields: Field::default(FieldOptions::EndingField),
                }
            },
 
            FieldOptions::ModelField => {
                return JsonData {
                    model: "verbs.models",
                    pk: 0,
                    fields: Field::default(FieldOptions::ModelField),
                }
            },

            FieldOptions::BaseField => {
                return JsonData {
                    model: "verbs.bases",
                    pk: 0,
                    fields: Field::default(FieldOptions::BaseField),
                }
            },

            FieldOptions::TenseField => {
                return JsonData {
                    model: "verbs.tenses",
                    pk: 0,
                    fields: Field::default(FieldOptions::TenseField),
                }
            },
 
            FieldOptions::SubjectField => {
                return JsonData {
                    model: "verbs.subjects",
                    pk: 0,
                    fields: Field::default(FieldOptions::SubjectField),
                }
            },
 
            FieldOptions::AuxiliaryField => {
                return JsonData {
                    model: "verbs.auxiliaries",
                    pk: 0,
                    fields: Field::default(FieldOptions::AuxiliaryField),
                }
            },
 
            FieldOptions::ConjugateField => {
                return JsonData {
                    model: "verbs.conjugates",
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugateField),
                }
            },
            
            FieldOptions::ConjugationField => {
                return JsonData {
                    model: "verbs.conjugations",
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugationField),
                }
            },
            
            FieldOptions::SentenceField => {
                return JsonData {
                    model: "verbs.sentences",
                    pk: 0,
                    fields: Field::default(FieldOptions::SentenceField),
                }
            },
        }
    }
}


