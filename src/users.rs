use argon2::{Argon2, PasswordHash};
use base64::Engine;
use password_hash::SaltString;
use rand::{thread_rng, RngCore};
use sqlx::{query, query_scalar, types::time::OffsetDateTime, SqlitePool};
use thiserror::Error;
use tokio::task::spawn_blocking;
use tracing::warn;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("password hashing error")]
    PasswordError(#[from] password_hash::errors::Error),
    #[error("SQL error")]
    SqlError(#[from] sqlx::Error),
    #[error("user with id={0} not found")]
    NoSuchUser(i64),
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

fn password_hash(password: String) -> password_hash::errors::Result<String> {
    let argon2 = Argon2::default();

    let salt_s = SaltString::generate(&mut thread_rng());
    let hash = PasswordHash::generate(argon2, password, salt_s.as_salt().as_str())?.serialize();
    Ok(hash.as_str().to_owned())
}

pub async fn add_user(
    pool: &SqlitePool,
    username: &str,
    password: String,
) -> Result<i64, UsersError> {
    let time = OffsetDateTime::now_utc();
    let hash = spawn_blocking(|| password_hash(password))
        .await
        .expect("Spawn blocking")?;
    let user_id = query!(
        "insert into users (name, password, created_at, updated_at) values (?, ?, ?, ?)",
        username,
        hash,
        time,
        time
    )
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(user_id)
}

pub async fn set_password(pool: &SqlitePool, id: i64, password: String) -> Result<(), UsersError> {
    let hash = spawn_blocking(|| password_hash(password))
        .await
        .expect("Spawn blocking")?;
    let rows_affected = query!("update users set password = ? where id = ?", hash, id)
        .execute(pool)
        .await?
        .rows_affected();
    if rows_affected == 1 {
        Ok(())
    } else if rows_affected == 0 {
        Err(UsersError::NoSuchUser(id))
    } else {
        panic!()
    }
}

pub async fn user_by_name(pool: &SqlitePool, username: &str) -> Result<Option<i64>, UsersError> {
    Ok(
        query_scalar!("select id from users where name = ?", username)
            .fetch_optional(pool)
            .await?,
    )
}
