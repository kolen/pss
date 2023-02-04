use argon2::{Argon2, PasswordHash};
use sqlx::{query, SqlitePool};
use tokio::task::spawn_blocking;
use tracing::warn;

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
