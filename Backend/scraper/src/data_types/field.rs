use serde::{Serialize, Deserialize};
use crate::data_types::field_options::{
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
    BaseField,
    TenseField,
    TenseSubfield,
    SubjectField,
    AuxiliaryField,
    ConjugateField,
    ConjugationField,
    SentenceField,
 };


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    LanguageField(LanguageField),
    GroupField(GroupField),
    EndingField(EndingField),
    ModelField(ModelField),
    BaseField(BaseField),
    TenseField(TenseField),
    SubjectField(SubjectField),
    AuxiliaryField(AuxiliaryField),
    ConjugateField(ConjugateField),
    ConjugationField(ConjugationField),
    SentenceField(SentenceField),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FieldOptions {
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
    BaseField,
    TenseField,
    SubjectField,
    AuxiliaryField,
    ConjugateField,
    ConjugationField,
    SentenceField,
}


impl Field {
    pub fn default(field_type: FieldOptions) -> Field {
        match field_type {
            FieldOptions::LanguageField => {
                let language_field = LanguageField {
                    language: String::new(),
                };
                return Field::LanguageField(language_field)
            },
            
            FieldOptions::GroupField => {
                let group_field = GroupField {
                    language: String::new(),
                    group: String::new(), 
                };
                return Field::GroupField(group_field)
            },

            FieldOptions::EndingField => {
                let ending_field = EndingField {
                    group: String::new(),
                    ending: String::new(),
                };
                return Field::EndingField(ending_field)
            },
           
            FieldOptions::ModelField => {
                let model_field = ModelField {
                    ending: String::new(),
                    model: String::new(),
                };
                return Field::ModelField(model_field)
            },

            FieldOptions::BaseField => {
                let base_field = BaseField {
                    rank: 0,
                    base: String::new(),
                    language: String::new(),
                };
                return Field::BaseField(base_field)
            },

            FieldOptions::TenseField => {
                let tense_subfield = TenseSubfield { major: String::new(), minor: String::new() };
                let tense_field = TenseField {
                    rank: 0,
                    tense: tense_subfield,
                    language: String::new(),
                };
                return Field::TenseField(tense_field)
            },

            FieldOptions::SubjectField => {
                let subject_field = SubjectField {
                    rank: 0,
                    subject: String::new(),
                    language: String::new(),
                };
                return Field::SubjectField(subject_field)
            },

            FieldOptions::AuxiliaryField => {
                let auxiliary_field = AuxiliaryField {
                    auxiliary: String::new(),
                    language: String::new(),
                };
                return Field::AuxiliaryField(auxiliary_field)
            },

            FieldOptions::ConjugateField => {
                let conjugate_field = ConjugateField {
                    rank: 0,
                    base: String::new(),
                    conjugate: String::new(),
                    model: String::new(),
                };
                return Field::ConjugateField(conjugate_field)
            },

            FieldOptions::ConjugationField => {
                let conjugation_field = ConjugationField {
                    rank: 0,
                    tense: String::new(),
                    subject: String::new(),
                    auxiliary: String::new(),
                    conjugate: String::new(),
                };
                return Field::ConjugationField(conjugation_field)
            },

            FieldOptions::SentenceField => {
                let sentence_field = SentenceField {
                    rank: 0,
                    conjugation: String::new(),
                    sentence: String::new(),
                    char_start: 0,
                    char_length: 0,
                };
                return Field::SentenceField(sentence_field);
            },
        } 
    }
}


