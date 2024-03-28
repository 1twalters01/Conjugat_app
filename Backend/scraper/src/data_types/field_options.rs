use serde::{Deserialize, Serialize};

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize, Clone)]
pub struct LanguageField {
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupField {
    pub language: String,
    pub group: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndingField {
    pub group: String,
    pub ending: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelField {
    pub ending: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseField {
    pub rank: i64,
    pub language: String,
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MajorTenseField {
    pub major_tense: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MinorTenseField {
    pub minor_tense: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TenseField {
    pub rank: i64,
    pub tense: TenseSubfield,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TenseSubfield {
    pub major: String,
    pub minor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParticleField {
    pub particle: String,
    pub language: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectField {
    pub rank: i64,
    pub language: String,
    pub subject: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuxiliaryField {
    pub language: String,
    pub auxiliary: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConjugateField {
    pub rank: i64,
    pub base: String,
    pub model: String,
    pub conjugate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConjugationField {
    pub rank: i64,
    pub tense: String,
    pub particle: String,
    pub subject: String,
    pub auxiliary: String,
    pub conjugate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentenceField {
    pub rank: i64,
    pub conjugation: String,
    pub sentence: String,
    pub char_start: i64,
    pub char_length: i64,
}
