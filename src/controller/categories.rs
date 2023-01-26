use crate::api_data::{Categories, Category, CategoryCreateRequest, CategoryUpdateRequest};
use crate::controller::utils::InternalServerErrorResultExt;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Result;
use axum::{Extension, Json};
use futures::stream::StreamExt;
use sqlx::types::time::OffsetDateTime;
use sqlx::{query, query_as, query_scalar, SqlitePool};

pub const SAMPLE_WORDS_COUNT: i32 = 5;

/// Basic information about category, including word count. Intermediate structure for outputting.
struct CategoryBasic {
    id: i64,
    name: Option<String>,
    num_words: i64,
}

async fn build_category(
    exec: impl sqlx::SqliteExecutor<'_>,
    category: CategoryBasic,
) -> sqlx::Result<Category> {
    let sample_words = query_scalar!(
        "select word from words where category_id = ? limit ?",
        category.id,
        SAMPLE_WORDS_COUNT
    )
    .fetch_all(exec)
    .await?;

    Ok(Category {
        id: category.id,
        name: category.name,
        num_words: category.num_words,
        sample_words,
    })
}

pub async fn list_categories(Extension(pool): Extension<SqlitePool>) -> Result<Json<Categories>> {
    let mut categories_basic = query_as!(
        CategoryBasic,
        "select categories.id, categories.name, cast(count(words.id) as integer) as num_words
        from categories
        left join words on categories.id = words.category_id group by categories.id"
    )
    .fetch(&pool);

    let mut categories: Vec<Category> = Vec::with_capacity(SAMPLE_WORDS_COUNT as usize);

    while let Some(category_r) = categories_basic.next().await {
        let category = category_r.into_500()?;
        categories.push(build_category(&pool, category).await.into_500()?);
    }

    Ok(Json(Categories { categories }))
}

pub async fn create_category(
    Extension(pool): Extension<SqlitePool>,
    Json(category_create): Json<CategoryCreateRequest>,
) -> Result<Json<Category>> {
    // FIXME: use user from auth
    let user_id = query_scalar!("select id from users limit 1")
        .fetch_one(&pool)
        .await
        .expect("Temporary requires only single user in database TODO");
    let current_time = OffsetDateTime::now_utc();
    let new_category_id = query!(
        "insert into categories(user_id, name, created_at, updated_at)
        values(?, ?, ?, ?)",
        user_id,
        category_create.name,
        current_time,
        current_time
    )
    .execute(&pool)
    .await
    .into_500()?
    .last_insert_rowid();

    Ok(Json(Category {
        id: new_category_id,
        name: category_create.name,
        num_words: 0,
        sample_words: vec![],
    }))
}

pub async fn update_category(
    Extension(pool): Extension<SqlitePool>,
    Path(category_id): Path<i64>,
    Json(category_update): Json<CategoryUpdateRequest>,
) -> Result<Json<Category>> {
    // FIXME: add auth

    let categories_updated = query!(
        "update categories set name = ? where id = ?",
        category_update.name,
        category_id
    )
    .execute(&pool)
    .await
    .into_500()?
    .rows_affected();

    if categories_updated == 0 {
        return Err((StatusCode::NOT_FOUND, "Category not found").into());
    }

    let category = query_as!(
        CategoryBasic,
        "select categories.id, categories.name, cast(count(words.id) as integer) as num_words
        from categories
        left join words on categories.id = words.category_id
        where categories.id = ? group by categories.id",
        category_id
    )
    .fetch_one(&pool)
    .await
    .into_500()?;

    Ok(Json(build_category(&pool, category).await.into_500()?))
}

pub async fn delete_category(
    Extension(pool): Extension<SqlitePool>,
    Path(category_id): Path<i64>,
) -> Result<()> {
    // FIXME: add auth
    // FIXME: refine logic of categories that can't be deleted

    let num_deleted = query!(
        "with deletable_categories as (
          select categories.id from categories
          left join words on words.category_id = categories.id
          group by categories.id having count(words.id) = 0
        )
        delete from categories where id = ?
          and exists (
            select * from deletable_categories where deletable_categories.id = id
          )",
        category_id
    )
    .execute(&pool)
    .await
    .into_500()?
    .rows_affected();

    if num_deleted == 1 {
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, "Category not found").into())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        api_data::{CategoryCreateRequest, CategoryUpdateRequest},
        test_utils::*,
    };
    use axum::{extract::Path, Extension, Json};
    use sqlx::query_scalar;

    use crate::api_data::Categories;

    #[tokio::test]
    async fn test_list_categories_basic() {
        let pool = test_database_pool().await;
        let user = add_test_user(&pool, "user").await;
        let category_id = add_test_category(&pool, user).await;
        for _ in 0..5 {
            add_test_word(&pool, category_id).await;
        }
        add_test_category(&pool, user).await;

        let Json(Categories { categories }) = super::list_categories(Extension(pool.clone()))
            .await
            .expect("successful response with list of categories");

        assert_eq!(categories.len(), 2);
        let long_category = categories
            .iter()
            .find(|c| c.num_words == 5)
            .expect("category exists with 5 words");
        assert_eq!(
            long_category.sample_words.len(),
            super::SAMPLE_WORDS_COUNT as usize
        );
        assert!(long_category
            .sample_words
            .iter()
            .all(|sample_word| sample_word.len() > 0));
    }

    #[tokio::test]
    async fn test_list_categories_empty() {
        let pool = test_database_pool().await;
        add_test_user(&pool, "user").await;
        let Json(Categories { categories }) = super::list_categories(Extension(pool.clone()))
            .await
            .expect("successful response with list of categories");
        assert!(categories.is_empty());
    }

    #[tokio::test]
    async fn test_create_category_basic() {
        let pool = test_database_pool().await;
        add_test_user(&pool, "user").await;

        let create_category = CategoryCreateRequest {
            name: Some("test".to_owned()),
        };
        let Json(category) = super::create_category(Extension(pool.clone()), Json(create_category))
            .await
            .expect("successful response");
        assert_eq!(category.name, Some("test".to_owned()));
        assert!(category.id > 0);
    }

    #[tokio::test]
    async fn test_create_category_no_name() {
        let pool = test_database_pool().await;
        add_test_user(&pool, "user").await;

        let create_category = CategoryCreateRequest { name: None };
        let Json(category) = super::create_category(Extension(pool.clone()), Json(create_category))
            .await
            .expect("successful response");
        assert_eq!(category.name, None);
        assert!(category.id > 0);
    }

    #[tokio::test]
    async fn test_update_category_basic() {
        let pool = test_database_pool().await;
        let user_id = add_test_user(&pool, "user").await;
        let category_id = add_test_category(&pool, user_id).await;
        add_test_word(&pool, category_id).await;

        let Json(category) = super::update_category(
            Extension(pool.clone()),
            Path(category_id),
            Json(CategoryUpdateRequest {
                name: Some("foo".to_owned()),
            }),
        )
        .await
        .expect("successful response");
        assert_eq!(category.name, Some("foo".to_owned()));
        assert_eq!(category.num_words, 1);
        assert_eq!(category.sample_words.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_category_basic() {
        let pool = test_database_pool().await;
        let user_id = add_test_user(&pool, "user").await;
        let category_id = add_test_category(&pool, user_id).await;

        super::delete_category(Extension(pool.clone()), Path(category_id))
            .await
            .expect("successful response");

        let has_category =
            query_scalar!("select count(*) from categories where id = ?", category_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(has_category, 0);
    }

    #[tokio::test]
    async fn test_delete_category_not_found() {
        let pool = test_database_pool().await;
        super::delete_category(Extension(pool.clone()), Path(1))
            .await
            .unwrap_err();
    }
}
