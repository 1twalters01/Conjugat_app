use serde::{Serialize, Deserialize};


#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize, Clone)]
pub struct LanguageField<'a> {
    language: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupField<'a> {
    language: &'a str,
    group: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndingField<'a> {
    group: &'a str,
    ending: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelField<'a> {
    ending: &'a str,
    model: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseField<'a> {
    rank: i64,
    language: &'a str,
    base: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TenseField<'a> {
    language: &'a str,
    tense: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectField<'a> {
    language: &'a str,
    subject: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuxiliaryField<'a> {
    language: &'a str,
    auxiliary: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConjugateField<'a> {
    base: &'a str,
    conjugate: &'a str,
    model: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConjugationField<'a> {
    rank: i64,
    tense: &'a str,
    subject: &'a str,
    auxiliary: &'a str,
    conjugate: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentenceField<'a> {
    rank: i64,
    conjugation: &'a str,
    sentence: &'a str,
    char_start: i64,
    char_length: i64,
}


