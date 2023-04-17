use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct IOError(io::Error);

impl From<io::Error> for IOError {
    fn from(err: io::Error) -> Self {
        IOError(err)
    }
}

impl IntoResponse for IOError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error reading file: {}", self.0),
        )
            .into_response()
    }
}
