use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse, Redirect, Response, Result},
    Extension, Form,
};
use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::users::authenticate_user_by_password;

use super::utils::InternalServerErrorResultExt;

#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String,
}

pub async fn login_page(Extension(handlebars): Extension<Arc<Handlebars<'_>>>) -> Result<Response> {
    Ok(Html(handlebars.render("login.hbs", &()).into_500()?).into_response())
}

pub async fn login_submit(
    Extension(pool): Extension<SqlitePool>,
    Extension(handlebars): Extension<Arc<Handlebars<'_>>>,
    Form(form_data): Form<LoginFormData>,
) -> Result<Response> {
    let opt_user_id = authenticate_user_by_password(&pool, &form_data.username, form_data.password)
        .await
        .into_500()?;

    match opt_user_id {
        Some(_user_id) => Ok(Redirect::temporary("/").into_response()),
        None => Ok(Html(
            handlebars
                .render(
                    "login.hbs",
                    &json!({"username": form_data.username, "password": ""}),
                )
                .into_500()?,
        )
        .into_response()),
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::{http, Extension};

    use super::login_page;
    use crate::make_handlebars;

    #[tokio::test]
    async fn test_login_page() {
        let handlebars = make_handlebars();
        let response = login_page(Extension(Arc::new(handlebars))).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(http::header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap(),
            "text/html; charset=utf-8"
        );
    }
}
