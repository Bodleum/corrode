use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// TODO: Use error_stack

pub struct IOError(pub io::Error);
impl From<io::Error> for IOError {
    fn from(value: io::Error) -> Self {
        Self(value)
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

pub struct FromUTF8Error(std::string::FromUtf8Error);
impl From<std::string::FromUtf8Error> for FromUTF8Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self(value)
    }
}
impl IntoResponse for FromUTF8Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Error: Not valid UTF-8.").into_response()
    }
}

pub struct AxumError(pub axum::Error);
impl From<axum::Error> for AxumError {
    fn from(value: axum::Error) -> Self {
        Self(value)
    }
}
impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

pub enum MiddlewareError {
    FromUTF8Error(FromUTF8Error),
    AxumError(AxumError),
}
impl From<FromUTF8Error> for MiddlewareError {
    fn from(value: FromUTF8Error) -> Self {
        Self::FromUTF8Error(value)
    }
}
impl From<std::string::FromUtf8Error> for MiddlewareError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUTF8Error(FromUTF8Error(value))
    }
}
impl From<AxumError> for MiddlewareError {
    fn from(value: AxumError) -> Self {
        Self::AxumError(value)
    }
}
impl From<axum::Error> for MiddlewareError {
    fn from(value: axum::Error) -> Self {
        Self::AxumError(AxumError(value))
    }
}
impl IntoResponse for MiddlewareError {
    fn into_response(self) -> Response {
        match self {
            Self::FromUTF8Error(err) => err.into_response(),
            Self::AxumError(err) => err.into_response(),
        }
    }
}
