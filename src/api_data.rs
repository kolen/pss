use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Category {
    pub id: i64,
    pub name: Option<String>,
    pub num_words: i64,
    pub sample_words: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryCreateRequest {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryUpdateRequest {
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Categories {
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
pub struct WordCreateRequest {
    pub word: String,
}

#[derive(Serialize)]
pub struct Word {
    pub id: i64,
    pub word: String,
}

#[derive(Serialize)]
pub struct Words {
    pub words: Vec<Word>,
}
