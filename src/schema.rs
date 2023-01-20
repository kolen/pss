use sqlx::SqlitePool;

/// Installs latest application schema to SQLite database if it isn't
/// already installed
pub async fn install_schema(pool: &SqlitePool) -> sqlx::Result<()> {
    let commands = include_str!("schema.sql").split(';');
    let mut transaction = pool.begin().await?;

    let tables_count =
        sqlx::query_scalar!("select count(*) as c from sqlite_schema where type = 'table'")
            .fetch_one(&mut transaction)
            .await?;
    if tables_count > 0 {
        return Ok(());
    }

    for command in commands {
        let command_trimmed = command.trim();
        if !command_trimmed.is_empty() {
            sqlx::query(command).execute(&mut transaction).await?;
        }
    }

    transaction.commit().await?;
    Ok(())
}
