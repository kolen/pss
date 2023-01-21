use crate::api_data::Category;
use crate::controller::utils::InternalServerErrorResultExt;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Result};
use axum::{Extension, Json};
use futures::stream::{self, StreamExt, TryStreamExt};
use sqlx::{query, query_scalar, SqlitePool};

const SAMPLE_WORDS_COUNT: i32 = 5;

pub async fn list_categories(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Category>>> {
    let mut categories_initial = query!(
        r#"select categories.id, categories.name, cast(count(words.id) as integer) as num_words
         from categories
         left join words on categories.id = words.category_id group by categories.id"#
    )
    .fetch(&pool);

    let mut categories: Vec<Category> = Vec::with_capacity(SAMPLE_WORDS_COUNT as usize);

    while let Some(category_r) = categories_initial.next().await {
        let category = category_r.into_500()?;

        let sample_words = query_scalar!(
            "select word from words where category_id = ? limit ?",
            category.id,
            SAMPLE_WORDS_COUNT
        )
        .fetch_all(&pool)
        .await
        .into_500()?;

        categories.push(Category {
            id: category.id,
            name: category.name,
            num_words: category.num_words,
            sample_words,
        });
    }

    Ok(Json(categories))
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

#[cfg(test)]
mod test {}
