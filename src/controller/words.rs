use crate::api_data::Word;
use axum::extract::Path;
use axum::response::Result;
use axum::Json;

pub async fn list_words(Path(category_id): Path<u64>) -> Result<Json<Vec<Word>>> {
    Ok(Json(vec![]))
}

pub async fn create_word(Path(category_id): Path<u64>) -> Result<Json<Word>> {
    Ok(Json(Word {
        id: 0,
        category_id: 0,
        word: "foo".to_string(),
    }))
}

pub async fn delete_word(Path(_category_id): Path<u64>, Path(word_id): Path<u64>) -> Result<()> {
    Ok(())
}
