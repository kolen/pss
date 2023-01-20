use crate::api_data::Category;
use axum::extract::Path;
use axum::response::Result;
use axum::Json;

pub async fn list_categories() -> Json<Vec<Category>> {
    Json(vec![])
}

pub async fn create_category() -> Result<Json<Category>> {
    Ok(Json(Category {
        id: 0,
        name: None,
        num_words: 0,
        sample_words: vec![],
    }))
}

pub async fn edit_category(Path(category_id): Path<u64>) -> Result<Json<Category>> {
    Ok(Json(Category {
        id: 0,
        name: None,
        num_words: 0,
        sample_words: vec![],
    }))
}

pub async fn delete_category(Path(category_id): Path<u64>) -> Result<()> {
    Ok(())
}
