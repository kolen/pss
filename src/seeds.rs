use anyhow::Result;
use rand::random;
use sqlx::{query, types::time::OffsetDateTime, SqlitePool};

use crate::users::add_user;

pub async fn install(pool: &SqlitePool) -> Result<()> {
    let user = add_user(&pool, &"user", "123".to_string()).await?;
    add_user(&pool, &"user1", "123".to_string()).await?;
    let current_time = OffsetDateTime::now_utc();

    let mut category_ids: Vec<i64> = Vec::new();
    for category_name in ["орнитология", "медицина", "кулинария"] {
        let category_id = query!(
            "insert into categories (user_id, name, created_at, updated_at) values(?, ?, ?, ?)",
            user,
            category_name,
            current_time,
            current_time
        )
        .execute(pool)
        .await?
        .last_insert_rowid();
        category_ids.push(category_id);
    }

    for word in [
        "снегирь",
        "спазм",
        "расстегай",
        "дефлегматор",
        "сипуха",
        "пентаграмма",
    ] {
        let category_id = category_ids[random::<usize>() % 2];
        query!(
            "insert into words (category_id, word, created_at, updated_at) values (?, ?, ?, ?)",
            category_id,
            word,
            current_time,
            current_time
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
