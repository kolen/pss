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
