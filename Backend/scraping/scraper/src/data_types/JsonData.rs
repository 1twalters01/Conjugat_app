#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonData {
    model: &str,
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
    ConjugationField(ConjugationField),
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
    ConjugationField,
    SentenceField,
}



#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize, Clone)]
struct LanguageField {
    language: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupField {
    language: &str,
    group: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EndingField {
    group: &str,
    ending: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModelField {
    ending: &str,
    model: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BaseField {
    rank: i64,
    language: &str,
    base: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TenseField {
    language: &str,
    tense: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubjectField {
    language: &str,
    subject: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuxiliaryField {
    language: &str,
    auxiliary: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConjugateField {
    base: &str,
    conjugate: &str,
    model: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConjugationField {
    rank: i64,
    tense: &str,
    subject: &str,
    auxiliary: &str,
    conjugate: &str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SentenceField {
    rank: i64,
    conjugation: &str,
    sentence: &str,
    charStart: i64,
    charLength: i64,
}



impl JsonData {
    fn default(field_type: FieldOptions) -> JsonData {
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






impl Field {
    fn default(field_type: FieldOptions) -> Field {
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
                    conjugation: ""),
                    sentence: "",
                    charStart: 0,
                    charLength: 0,
                }
            },
        } 
    }
}
