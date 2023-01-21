use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Category {
    pub id: i64,
    pub name: Option<String>,
    pub num_words: i64,
    pub sample_words: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateWord {
    pub word: String,
}

#[derive(Serialize)]
pub struct Word {
    pub id: i64,
    pub category_id: i64,
    pub word: String,
}
