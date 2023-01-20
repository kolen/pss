use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Category {
    pub id: u64,
    pub name: Option<String>,
    pub num_words: u64,
    pub sample_words: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateWord {
    pub word: String,
}

#[derive(Serialize)]
pub struct Word {
    pub id: u64,
    pub category_id: u64,
    pub word: String,
}
