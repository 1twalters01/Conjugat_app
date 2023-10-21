use serde::{Serialize, Deserialize};
use crate::data_types::FieldOptions::{
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
};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field<'a> {
    #[serde(borrow)]
    LanguageField(LanguageField<'a>),
    GroupField(GroupField<'a>),
    EndingField(EndingField<'a>),
    ModelField(ModelField<'a>),
    BaseField(BaseField<'a>),
    TenseField(TenseField<'a>),
    SubjectField(SubjectField<'a>),
    AuxiliaryField(AuxiliaryField<'a>),
    ConjugateField(ConjugateField<'a>),
    ConjugationField(ConjugationField<'a>),
    SentenceField(SentenceField<'a>),
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


impl<'a> Field<'a> {
    pub fn default(field_type: FieldOptions) -> Field<'a> {
        match field_type {
            FieldOptions::LanguageField => {
                let language_field = LanguageField {
                    language: "",
                };
                return Field::LanguageField(language_field)
            },
            
            FieldOptions::GroupField => {
                let group_field = GroupField {
                    language: "",
                    group: "", 
                };
                return Field::GroupField(group_field)
            },

            FieldOptions::EndingField => {
                let ending_field = EndingField {
                    group: "",
                    ending: "",
                };
                return Field::EndingField(ending_field)
            },
           
            FieldOptions::ModelField => {
                let model_field = ModelField {
                    ending: "",
                    model: "",
                };
                return Field::ModelField(model_field)
            },

            FieldOptions::BaseField => {
                let base_field = BaseField {
                    rank: 0,
                    base: "",
                    language: "",
                };
                return Field::BaseField(base_field)
            },

            FieldOptions::TenseField => {
                let tense_field = TenseField {
                    tense: "",
                    language: "",
                };
                return Field::TenseField(tense_field)
            },

            FieldOptions::SubjectField => {
                let subject_field = SubjectField {
                    subject: "",
                    language: "",
                };
                return Field::SubjectField(subject_field)
            },

            FieldOptions::AuxiliaryField => {
                let auxiliary_field = AuxiliaryField {
                    auxiliary: "",
                    language: "",
                };
                return Field::AuxiliaryField(auxiliary_field)
            },

            FieldOptions::ConjugateField => {
                let conjugate_field = ConjugateField {
                    base: "",
                    conjugate: "",
                    model: "",
                };
                return Field::ConjugateField(conjugate_field)
            },

            FieldOptions::ConjugationField => {
                let conjugation_field = ConjugationField {
                    rank: 0,
                    tense: "",
                    subject: "",
                    auxiliary: "",
                    conjugate: "",
                };
                return Field::ConjugationField(conjugation_field)
            },

            FieldOptions::SentenceField => {
                let sentence_field = SentenceField {
                    rank: 0,
                    conjugation: "",
                    sentence: "",
                    char_start: 0,
                    char_length: 0,
                };
                return Field::SentenceField(sentence_field);
            },
        } 
    }
}


