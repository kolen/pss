use axum::RequestPartsExt;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Extension,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use sqlx::SqlitePool;
use tracing::error;

use crate::users::get_session_user;

const COOKIE_NAME: &str = "pss_session";

pub struct SessionUser(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies: CookieJar =
            CookieJar::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    error!("Cookies error: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Cookies error")
                })?;
        if let Some(session_secret) = cookies.get(COOKIE_NAME) {
            let Extension(pool) = parts
                .extract::<Extension<SqlitePool>>()
                .await
                .expect("Extract database pool");
            let session_secret_s = session_secret.value();

            if let Some(user_id) =
                get_session_user(&pool, &session_secret_s)
                    .await
                    .map_err(|e| {
                        error!("Session lookup error: {}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, "Session lookup error")
                    })?
            {
                Ok(SessionUser(user_id))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid session"))
            }
        } else {
            Err((StatusCode::UNAUTHORIZED, "Not authenticated"))
        }
    }
}

pub fn session_cookie(secret: String) -> CookieJar {
    let cookie = Cookie::build(COOKIE_NAME, secret).http_only(true).finish();
    CookieJar::new().add(cookie)
}
