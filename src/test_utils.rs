use sqlx::{query, SqlitePool};

use crate::schema::install_schema;

pub async fn test_database_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("create pool with in-memory sqlite database");
    install_schema(&pool).await.expect("install schema");
    pool
}

pub async fn add_test_user(pool: &SqlitePool, name: &str) -> i64 {
    query!(
        "insert into users(name, created_at, updated_at) values (?, '2022-01-01', '2022-01-01')",
        name
    )
    .execute(pool)
    .await
    .expect("add test user")
    .last_insert_rowid()
}

pub async fn add_test_category(pool: &SqlitePool, user_id: i64) -> i64 {
    query!("insert into categories(user_id, name, created_at, updated_at) values (?, null, '2022-01-01', '2022-01-01')", user_id)
        .execute(pool)
        .await
        .expect("add test category")
        .last_insert_rowid()
}

pub async fn add_test_word(pool: &SqlitePool, category_id: i64) -> i64 {
    query!("insert into words(category_id, word, created_at, updated_at) values (?, 'word'+hex(randomblob(8)), '2022-01-01', '2022-01-01')", category_id)
        .execute(pool)
        .await
        .expect("add test word")
        .last_insert_rowid()
}
