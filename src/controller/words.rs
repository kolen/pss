use crate::api_data::{Word, WordCreateRequest, Words};
use crate::auth::SessionUser;
use crate::controller::utils::InternalServerErrorResultExt;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use sqlx::types::time::OffsetDateTime;
use sqlx::{query, query_as, query_scalar, SqlitePool};

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

pub async fn create_word(
    Extension(pool): Extension<SqlitePool>,
    Path(category_id): Path<i64>,
    SessionUser(user_id): SessionUser,
    Json(word_create): Json<WordCreateRequest>,
) -> Result<Json<Word>> {
    if let Some(_) = query_scalar!("select id from categories where user_id = ?", user_id)
        .fetch_optional(&pool)
        .await
        .into_500()?
    {
        let current_time = OffsetDateTime::now_utc();
        let word_id = query!(
            "insert into words (category_id, word, created_at, updated_at) values (?, ?, ?, ?)",
            category_id,
            word_create.word,
            current_time,
            current_time
        )
        .execute(&pool)
        .await
        .into_500()?
        .last_insert_rowid();
        Ok(Json(Word {
            id: word_id,
            word: word_create.word.to_owned(),
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Category not found").into())
    }
}

pub async fn delete_word(
    Extension(pool): Extension<SqlitePool>,
    Path(category_id): Path<i64>,
    Path(word_id): Path<i64>,
    SessionUser(user_id): SessionUser,
) -> Result<()> {
    let affected_rows = query!(
        "with deletable_words as (
          select words.id from words
          join categories on categories.id = words.category_id
          where categories.user_id = ?
        )
        delete from words where id = ?
          and exists (select * from deletable_words where deletable_words.id = id)
          and category_id = ?",
        user_id,
        word_id,
        category_id
    )
    .execute(&pool)
    .await
    .into_500()?
    .rows_affected();

    if affected_rows == 1 {
        Ok(())
    } else if affected_rows == 0 {
        Err((StatusCode::NOT_FOUND, "Not found").into())
    } else {
        panic!("More than one word deleted, shouldn't happen");
    }
}

#[cfg(test)]
mod test {
    use axum::{extract::Path, Extension, Json};

    use crate::{api_data::WordCreateRequest, auth::SessionUser, test_utils::*};

    #[tokio::test]
    async fn test_list_words_basic() {
        let pool = test_database_pool().await;
        let user = add_test_user(&pool, "user").await;
        let category = add_test_category(&pool, user).await;
        for _ in 0..5 {
            add_test_word(&pool, category).await;
        }

        let Json(words) = super::list_words(Extension(pool), Path(category), SessionUser(user))
            .await
            .expect("successful response");

        assert_eq!(words.words.len(), 5);
        assert!(words.words.first().unwrap().id > 0);
    }

    #[tokio::test]
    async fn test_create_word_basic() {
        let pool = test_database_pool().await;
        let user = add_test_user(&pool, "user").await;
        let category = add_test_category(&pool, user).await;

        let Json(word) = super::create_word(
            Extension(pool),
            Path(category),
            SessionUser(user),
            Json(WordCreateRequest {
                word: "foo".to_owned(),
            }),
        )
        .await
        .expect("successful response");

        assert!(word.id > 0);
        assert_eq!("foo", word.word);
    }

    #[tokio::test]
    async fn test_create_word_other_users_category() {
        let pool = test_database_pool().await;
        let user1 = add_test_user(&pool, "user1").await;
        let user2 = add_test_user(&pool, "user2").await;
        let category = add_test_category(&pool, user1).await;

        super::create_word(
            Extension(pool),
            Path(category),
            SessionUser(user2),
            Json(WordCreateRequest {
                word: "foo".to_owned(),
            }),
        )
        .await
        .expect_err("unsuccessful response");
        // TODO: check response code somehow
    }

    #[tokio::test]
    async fn test_delete_word_basic() {
        let pool = test_database_pool().await;
        let user = add_test_user(&pool, "user").await;
        let category = add_test_category(&pool, user).await;
        let word = add_test_word(&pool, category).await;

        super::delete_word(
            Extension(pool),
            Path(category),
            Path(word),
            SessionUser(user),
        )
        .await
        .expect("successful response");
    }

    #[tokio::test]
    async fn test_delete_word_another_user() {
        let pool = test_database_pool().await;
        let user1 = add_test_user(&pool, "user1").await;
        let user2 = add_test_user(&pool, "user2").await;
        let category = add_test_category(&pool, user1).await;
        let word = add_test_word(&pool, category).await;

        super::delete_word(
            Extension(pool),
            Path(category),
            Path(word),
            SessionUser(user2),
        )
        .await
        .expect_err("unsuccessful response");
        // TODO: check response code somehow
    }
}
