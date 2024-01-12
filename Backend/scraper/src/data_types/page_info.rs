pub struct PageInfo {
    pub metadata: PageMetadata,

    pub tenses: Vec<Tenses>,
    pub subjects: Vec<String>,
    pub auxiliaries: Vec<String>,
    pub conjugates: Vec<Vec<String>>,
}

struct PageMetadata {
    pub language: String,
    pub model: String,
    pub base: String,
    pub auxiliary: Vec<String>,
    pub forms: Vec<String>,
    pub similar_verbs: Vec<String>,
    pub other_verbs: Vec<String>,
}

struct Tenses {
    major: String,
    minor: String,
}
