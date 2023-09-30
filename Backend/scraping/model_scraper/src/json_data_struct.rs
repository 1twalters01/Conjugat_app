#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonData {
    model: String,
    pk: i64,
    fields: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Field {
    LanguageField(LanguageField),
    GroupField(GroupField),
    EndingField(EndingField),
    ModelField(ModelField),
    BaseField(BaseField),
    TenseField(TenseField),
    SubjectField(SubjectField),
    AuxiliaryField(AuxiliaryField),
    ConjugateField(ConjugateField),
    SentenceField(SentenceField),
}




#[derive(Debug, Serialize, Deserialize, Clone)]
enum FieldOptions {
    LanguageField,
    GroupField,
    EndingField,
    ModelField,
    BaseField,
    TenseField,
    SubjectField,
    AuxiliaryField,
    ConjugateField,
    SentenceField,
}




#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize, Clone)]
struct LanguageField {
    language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupField {
    language: String,
    group: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EndingField {
    group: String,
    ending: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModelField {
    ending: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BaseField {
    rank: i64,
    language: String,
    base: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TenseField {
    language: String,
    tense: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubjectField {
    language: String,
    subject: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuxiliaryField {
    language: String,
    auxiliary: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConjugateField {
    base: String,
    conjugate: String,
    model: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct SentenceField {
    rank: i64,
    tense: String,
    subject: String,
    auxiliary: String,
    conjugate: String
}



