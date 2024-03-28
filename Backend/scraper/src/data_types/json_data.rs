use crate::data_types::{
    field::{Field, FieldOptions},
    field_options::{
        AuxiliaryField, BaseField, ConjugateField, ConjugationField, EndingField, GroupField,
        LanguageField, MajorTenseField, MinorTenseField, ModelField, ParticleField, SentenceField,
        SubjectField, TenseField, TenseSubfield,
    },
};
use serde::{Deserialize, Serialize};

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
            }

            FieldOptions::GroupField => {
                return JsonData {
                    model: String::from("verbs.groups"),
                    pk: 0,
                    fields: Field::default(FieldOptions::GroupField),
                }
            }

            FieldOptions::EndingField => {
                return JsonData {
                    model: String::from("verbs.endings"),
                    pk: 0,
                    fields: Field::default(FieldOptions::EndingField),
                }
            }

            FieldOptions::ModelField => {
                return JsonData {
                    model: String::from("verbs.models"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ModelField),
                }
            }

            FieldOptions::BaseField => {
                return JsonData {
                    model: String::from("verbs.bases"),
                    pk: 0,
                    fields: Field::default(FieldOptions::BaseField),
                }
            }

            FieldOptions::MajorTenseField => {
                return JsonData {
                    model: String::from("verbs.major_tenses"),
                    pk: 0,
                    fields: Field::default(FieldOptions::MajorTenseField),
                }
            }

            FieldOptions::MinorTenseField => {
                return JsonData {
                    model: String::from("verbs.minor_tenses"),
                    pk: 0,
                    fields: Field::default(FieldOptions::MinorTenseField),
                }
            }

            FieldOptions::TenseField => {
                return JsonData {
                    model: String::from("verbs.tenses"),
                    pk: 0,
                    fields: Field::default(FieldOptions::TenseField),
                }
            }

            FieldOptions::ParticleField => {
                return JsonData {
                    model: String::from("verbs.particles"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ParticleField),
                }
            }

            FieldOptions::SubjectField => {
                return JsonData {
                    model: String::from("verbs.subjects"),
                    pk: 0,
                    fields: Field::default(FieldOptions::SubjectField),
                }
            }

            FieldOptions::AuxiliaryField => {
                return JsonData {
                    model: String::from("verbs.auxiliaries"),
                    pk: 0,
                    fields: Field::default(FieldOptions::AuxiliaryField),
                }
            }

            FieldOptions::ConjugateField => {
                return JsonData {
                    model: String::from("verbs.conjugates"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugateField),
                }
            }

            FieldOptions::ConjugationField => {
                return JsonData {
                    model: String::from("verbs.conjugations"),
                    pk: 0,
                    fields: Field::default(FieldOptions::ConjugationField),
                }
            }

            FieldOptions::SentenceField => {
                return JsonData {
                    model: String::from("verbs.sentences"),
                    pk: 0,
                    fields: Field::default(FieldOptions::SentenceField),
                }
            }
        }
    }
}

pub fn create_json_data_vec_from_vec_vec_string(
    data_vec_vec: &Vec<Vec<String>>,
    field_type: FieldOptions,
) -> Vec<JsonData> {
    let mut json_data: Vec<JsonData> = Vec::new();
    let mut primary_key: i64 = 0;

    for (_index2, data) in data_vec_vec.into_iter().enumerate() {
        primary_key = primary_key + 1;

        let field: Field = match field_type {
            FieldOptions::LanguageField => {
                let language_field = LanguageField {
                    language: data[0].clone(),
                };
                Field::LanguageField(language_field)
            }

            FieldOptions::GroupField => {
                let group_field = GroupField {
                    language: data[0].clone(),
                    group: data[1].clone(),
                };
                Field::GroupField(group_field)
            }

            FieldOptions::EndingField => {
                let ending_field = EndingField {
                    group: data[0].clone(),
                    ending: data[1].clone(),
                };
                Field::EndingField(ending_field)
            }

            FieldOptions::ModelField => {
                let model_field = ModelField {
                    ending: data[0].clone(),
                    model: data[1].clone(),
                };
                Field::ModelField(model_field)
            }

            FieldOptions::BaseField => {
                let base_field = BaseField {
                    rank: data[0].parse::<i64>().unwrap(),
                    language: data[1].clone(),
                    base: data[2].clone(),
                };
                Field::BaseField(base_field)
            }

            FieldOptions::MajorTenseField => {
                let major_tense_field = MajorTenseField {
                    language: data[0].clone(),
                    major_tense: data[1].clone(),
                };
                Field::MajorTenseField(major_tense_field)
            }

            FieldOptions::MinorTenseField => {
                let minor_tense_field = MinorTenseField {
                    language: data[0].clone(),
                    minor_tense: data[1].clone(),
                };
                Field::MinorTenseField(minor_tense_field)
            }

            FieldOptions::TenseField => {
                let tense_subfield = TenseSubfield {
                    major: data[1].clone(),
                    minor: data[2].clone(),
                };
                let tense_field = TenseField {
                    rank: data[0].parse::<i64>().unwrap(),
                    tense: tense_subfield,
                };
                Field::TenseField(tense_field)
            }

            FieldOptions::ParticleField => {
                let particle_field = ParticleField {
                    language: data[0].clone(),
                    particle: data[1].clone(),
                };
                Field::ParticleField(particle_field)
            }

            FieldOptions::SubjectField => {
                let subject_field = SubjectField {
                    rank: data[0].parse::<i64>().unwrap(),
                    language: data[1].clone(),
                    subject: data[2].clone(),
                };
                Field::SubjectField(subject_field)
            }

            FieldOptions::AuxiliaryField => {
                let auxiliary_field = AuxiliaryField {
                    language: data[0].clone(),
                    auxiliary: data[1].clone(),
                };
                Field::AuxiliaryField(auxiliary_field)
            }

            FieldOptions::ConjugateField => {
                let conjugate_field = ConjugateField {
                    rank: data[0].parse::<i64>().unwrap(),
                    base: data[1].clone(),
                    model: data[2].clone(),
                    conjugate: data[3].clone(),
                };
                Field::ConjugateField(conjugate_field)
            }

            // Need a different top type to Vec<Vec<&str>>
            FieldOptions::ConjugationField => {
                let conjugation_field = ConjugationField {
                    rank: data[0].parse::<i64>().unwrap(),
                    tense: data[1].clone(),
                    particle: data[2].clone(),
                    subject: data[3].clone(),
                    auxiliary: data[4].clone(),
                    conjugate: data[5].clone(),
                };
                Field::ConjugationField(conjugation_field)
            }

            // Need a different top type to Vec<Vec<&str>>
            FieldOptions::SentenceField => {
                let sentence_field = SentenceField {
                    rank: data[0].parse::<i64>().unwrap(),
                    conjugation: data[1].clone(),
                    sentence: data[2].clone(),
                    char_length: data[3].parse::<i64>().unwrap(),
                    char_start: data[4].parse::<i64>().unwrap(),
                };
                Field::SentenceField(sentence_field)
            }
        };

        let target_data = JsonData {
            pk: primary_key,
            fields: field,
            ..JsonData::default(&field_type)
        };

        json_data.push(target_data);
    }

    return json_data;
}
