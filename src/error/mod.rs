use std::{error::Error, fmt};

use axum::response::IntoResponse;
use error_stack::Report;
use hyper::StatusCode;

// TODO: Use error_stack

#[derive(Debug)]
pub struct IOError(pub std::io::Error);
impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("IO Error! {}", self.0))
    }
}
impl Error for IOError {}
impl IOError {
    pub fn kind(&self) -> std::io::ErrorKind {
        self.0.kind()
    }
}

#[derive(Debug)]
pub struct FromUTF8Error(pub std::string::FromUtf8Error);
impl fmt::Display for FromUTF8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("UTF-8 Error: {}", self.0))
    }
}
impl Error for FromUTF8Error {}

#[derive(Debug)]
pub enum PathError {
    StripPrefixError(std::path::StripPrefixError),
    FileNameError(String),
    UnicodeError(String),
}
impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Self::StripPrefixError(err) => err.to_string(),
            Self::FileNameError(err) => err.clone(),
            Self::UnicodeError(err) => err.clone(),
        };
        f.write_fmt(format_args!("Path Error! {}", s))
    }
}
impl Error for PathError {}

#[derive(Debug)]
pub struct DirError;
impl fmt::Display for DirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Directory error!")
    }
}
impl Error for DirError {}

#[derive(Debug)]
pub struct PageError;
impl fmt::Display for PageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error while getting current page!")
    }
}
impl Error for PageError {}

#[derive(Debug)]
pub struct AxumError(pub axum::Error);
impl fmt::Display for AxumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Axum Error! {}", self.0))
    }
}
impl Error for AxumError {}

#[derive(Debug)]
pub struct MiddlewareError;
impl fmt::Display for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error in middleware!")
    }
}
impl Error for MiddlewareError {}

#[derive(Debug)]
pub struct PageReport(pub Report<PageError>);
impl From<Report<PageError>> for PageReport {
    fn from(value: Report<PageError>) -> Self {
        Self(value)
    }
}
impl IntoResponse for PageReport {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ansi_to_html::convert_escaped(&format!("{:#?}", self.0))
                .unwrap_or(format!("{}", self.0))
                .replace("\n", "<br />\n"),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct MiddlewareReport(pub Report<MiddlewareError>);
impl From<Report<MiddlewareError>> for MiddlewareReport {
    fn from(value: Report<MiddlewareError>) -> Self {
        Self(value)
    }
}
impl IntoResponse for MiddlewareReport {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ansi_to_html::convert_escaped(&format!("{:#?}", self.0))
                .unwrap_or(format!("{}", self.0))
                .replace("\n", "<br />\n"),
        )
            .into_response()
    }
}
