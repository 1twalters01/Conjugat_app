use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageInfo {
    pub metadata: PageMetadata,

    pub major_tenses: Vec<String>,
    pub minor_tenses: Vec<String>,
    pub subjects: Vec<String>,
    pub auxiliaries: Vec<String>,
    pub conjugates: Vec<String>,
    pub particles: Vec<String>,
    // pub other: Vec<String>,
    pub tenses: Vec<Tense>,
    pub phrases: Vec<Vec<Phrase>>,
}

impl PageInfo {
    pub fn new() -> PageInfo {
        let page_info = PageInfo {
            metadata: PageMetadata::new(),
            major_tenses: Vec::new(),
            minor_tenses: Vec::new(),
            subjects: Vec::new(),
            auxiliaries: Vec::new(),
            conjugates: Vec::new(),
            particles: Vec::new(),
            // other: Vec::new(),
            tenses: Vec::new(),
            phrases: Vec::new(),
        };

        return page_info;
    }
}

// // make this the commented out version instead
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Phrase  {
//     pub subject: String,
//     pub auxiliary: String,
//     pub conjugate: String,
// }
//
// impl Phrase {
//     pub fn new() -> Phrase {
//         let phrase = Phrase {
//             subject: String::new(),
//             auxiliary: String::new(),
//             conjugate: String::new(),
//         };
//
//         return phrase;
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phrase  {
    pub phrase: Vec<String>,
    pub subjects: Vec<String>,
    pub auxiliaries: Vec<String>,
    pub conjugates: Vec<String>,
    pub particles: Vec<String>,
    pub other: Vec<String>,
}

impl Phrase {
    pub fn new() -> Phrase {
        let phrase = Phrase {
            phrase: Vec::new(),
            subjects: Vec::new(),
            auxiliaries: Vec::new(),
            conjugates: Vec::new(),
            particles: Vec::new(),
            other: Vec::new(),
        };

        return phrase;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageMetadata {
    pub language: String,
    pub rank: String,
    pub model: String,
    pub base: String,
    pub auxiliary: Vec<String>,
    pub forms: Vec<String>,
    pub similar_verbs: Vec<String>,
    pub other_verbs: Vec<String>,
}

impl PageMetadata {
    pub fn new() -> PageMetadata {
        let page_metadata = PageMetadata {
            language: String::new(),
            rank: String::new(),
            model: String::new(),
            base: String::new(),
            auxiliary: Vec::new(),
            forms: Vec::new(),
            similar_verbs: Vec::new(),
            other_verbs: Vec::new(),
        };

        return page_metadata;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tense {
    pub major: String,
    pub minor: String,
}

impl Tense {
    pub fn new() -> Tense {
        let tense = Tense {
            major: String::new(),
            minor: String::new(),
        };

        return tense;
    }
}
