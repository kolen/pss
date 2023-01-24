use crate::api_data::{Categories, Category};
use crate::controller::utils::InternalServerErrorResultExt;
use axum::extract::Path;
use axum::response::Result;
use axum::{Extension, Json};
use futures::stream::StreamExt;
use sqlx::{query, query_scalar, SqlitePool};

pub const SAMPLE_WORDS_COUNT: i32 = 5;

pub async fn list_categories(Extension(pool): Extension<SqlitePool>) -> Result<Json<Categories>> {
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

    Ok(Json(Categories { categories }))
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
mod test {
    use axum::{Extension, Json};
    use sqlx::{query, SqlitePool};

    use crate::{api_data::Categories, schema::install_schema};

    async fn test_database_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("create pool with in-memory sqlite database");
        install_schema(&pool).await.expect("install schema");
        pool
    }

    async fn add_test_user(pool: &SqlitePool, name: &str) -> i64 {
        query!("insert into users(name, created_at, updated_at) values (?, '2022-01-01', '2022-01-01')", name)
            .execute(pool)
            .await
            .expect("add test user")
            .last_insert_rowid()
    }

    async fn add_test_category(pool: &SqlitePool, user_id: i64) -> i64 {
        query!("insert into categories(user_id, name, created_at, updated_at) values (?, null, '2022-01-01', '2022-01-01')", user_id)
            .execute(pool)
            .await
            .expect("add test category")
            .last_insert_rowid()
    }

    async fn add_test_word(pool: &SqlitePool, category_id: i64) -> i64 {
        query!("insert into words(category_id, word, created_at, updated_at) values (?, 'word'+hex(randomblob(8)), '2022-01-01', '2022-01-01')", category_id)
            .execute(pool)
            .await
            .expect("add test word")
            .last_insert_rowid()
    }

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
}
