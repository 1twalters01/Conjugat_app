#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageInfo {
    pub metadata: PageMetadata,

    pub tenses: Vec<Tense>,
    pub subjects: Vec<String>,
    pub auxiliaries: Vec<String>,
    pub conjugates: Vec<Vec<String>>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct PageMetadata {
    pub language: String,
    pub rank: String,
    pub model: String,
    pub base: String,
    pub auxiliary: Vec<String>,
    pub forms: Vec<String>,
    pub similar_verbs: Vec<String>,
    pub other_verbs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tense {
    pub major: Option<String>,
    pub minor: Option<String>,
}


pub trait PageInfoIterator: Iterator {}
