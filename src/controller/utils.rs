use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub trait InternalServerErrorResponseExt {
    fn to_500(&self) -> Response;
}

impl<T: std::fmt::Display> InternalServerErrorResponseExt for T {
    fn to_500(&self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self),
        )
            .into_response()
    }
}

pub trait InternalServerErrorResultExt<T> {
    fn into_500(self) -> Result<T, Response>;
}

impl<T, E: InternalServerErrorResponseExt> InternalServerErrorResultExt<T> for Result<T, E> {
    fn into_500(self) -> Result<T, Response> {
        self.map_err(|e| e.to_500())
    }
}
