use crate::api_data::{Word, Words};
use crate::auth::SessionUser;
use crate::controller::utils::InternalServerErrorResultExt;
use axum::extract::Path;
use axum::response::Result;
use axum::{Extension, Json};
use sqlx::{query, query_as, SqlitePool};

pub async fn list_words(
    Extension(pool): Extension<SqlitePool>,
    Path(category_id): Path<i64>,
    SessionUser(user_id): SessionUser,
) -> Result<Json<Words>> {
    let words = query_as!(
        Word,
        r#"select w.id as "id!", w.word from words w
        join categories c on w.category_id = c.id
        where w.category_id = ? and c.user_id = ?"#,
        category_id,
        user_id
    )
    .fetch_all(&pool)
    .await
    .into_500()?;

    Ok(Json(Words { words }))
}

pub async fn create_word(Path(category_id): Path<u64>) -> Result<Json<Word>> {
    Ok(Json(Word {
        id: 0,
        word: "foo".to_string(),
    }))
}

pub async fn delete_word(Path(_category_id): Path<u64>, Path(word_id): Path<u64>) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use axum::{extract::Path, Extension, Json};

    use crate::{auth::SessionUser, test_utils::*};

    #[tokio::test]
    async fn test_list_words_basic() {
        let pool = test_database_pool().await;
        let user = add_test_user(&pool, "user").await;
        let category_id = add_test_category(&pool, user).await;
        for _ in 0..5 {
            add_test_word(&pool, category_id).await;
        }

        let Json(words) = super::list_words(Extension(pool), Path(category_id), SessionUser(user))
            .await
            .expect("successful response");

        assert_eq!(words.words.len(), 5);
        assert!(words.words.first().unwrap().id > 0);
    }
}
