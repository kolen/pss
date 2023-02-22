use argon2::{Argon2, PasswordHash};
use base64::Engine;
use rand::{thread_rng, RngCore};
use sqlx::{query, query_scalar, types::time::OffsetDateTime, SqlitePool};
use tokio::task::spawn_blocking;
use tracing::warn;

enum PasswordOrSqlError {
    PasswordError(password_hash::errors::Error),
    SqlError(sqlx::Error),
}

fn check_password_hash(password_hash: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(password_hash) {
        Ok(hash) => {
            let check = hash.verify_password(&[&argon2], password);
            match check {
                Ok(()) => true,
                Err(password_hash::errors::Error::Password) => false,
                Err(_) => panic!("Password check error"), // Shouldn't happen
            }
        }
        Err(error) => {
            warn!("Password hash error: {}", error);
            false
        }
    }
}

async fn check_password_hash_in_worker(password_hash: String, password: String) -> bool {
    spawn_blocking(move || check_password_hash(&password_hash, &password))
        .await
        .expect("run password hash checking operation")
}

/// Try to authenticate user by password, returning user id on success
pub async fn authenticate_user_by_password(
    pool: &SqlitePool,
    username: &str,
    password: String,
) -> sqlx::Result<Option<i64>> {
    let record = query!("select id, password from users where name = ?", username)
        .fetch_optional(pool)
        .await?;
    match record {
        None => Ok(None),
        Some(record1) => {
            match record1.password {
                Some(record_password) => {
                    if check_password_hash_in_worker(record_password, password).await {
                        Ok(Some(record1.id))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None), // Password column is NULL means password is disabled
            }
        }
    }
}

fn new_session_secret() -> String {
    let mut secret = [0u8; 12];
    thread_rng().fill_bytes(&mut secret);
    base64::engine::general_purpose::URL_SAFE.encode(&secret)
}

pub async fn create_session(
    pool: &SqlitePool,
    user_id: i64,
    user_agent: &str,
) -> sqlx::Result<String> {
    let time = OffsetDateTime::now_utc();
    let secret = new_session_secret();
    query!("insert into sessions (user_id, secret, created_user_agent, created_at, last_used_at) values (?, ?, ?, ?, ?)",
           user_id, secret, user_agent, time, time).execute(pool).await?;
    Ok(secret)
}

pub async fn get_session_user(pool: &SqlitePool, secret: &str) -> sqlx::Result<Option<i64>> {
    query_scalar!("select user_id from sessions where secret = ?", secret)
        .fetch_optional(pool)
        .await
}

pub async fn add_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<i64, PasswordOrSqlError> {
    let argon2 = Argon2::default();
    let salt = "example"; // TODO: generate randomly, use Salt::RECOMMENDED_LENGTH
    let hash = PasswordHash::generate(argon2, password, salt)
        .map_err(|e| PasswordOrSqlError::PasswordError(e))?
        .serialize();
    let hash_s = hash.as_str();
    let time = OffsetDateTime::now_utc();
    let user_id = query!(
        "insert into users (name, password, created_at, updated_at) values (?, ?, ?, ?)",
        username,
        hash_s,
        time,
        time
    )
    .execute(pool)
    .await
    .map_err(|e| PasswordOrSqlError::SqlError(e))?
    .last_insert_rowid();
    Ok(user_id)
}
