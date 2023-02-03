use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse, Response, Result},
    Extension,
};
use handlebars::Handlebars;

use super::utils::InternalServerErrorResultExt;

pub async fn login_page(Extension(handlebars): Extension<Arc<Handlebars<'_>>>) -> Result<Response> {
    Ok(Html(handlebars.render("login.hbs", &()).into_500()?).into_response())
}

pub async fn login_submit() -> Result<String> {
    Ok("".to_owned()) // TODO
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
