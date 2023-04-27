mod parse;
pub use parse::*;

use std::{path::StripPrefixError, string::FromUtf8Error};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tokio::io;

// TODO: Maybe use anyhow in the IntoResponse bits to show error better?

/// Represents an error occurring while dismantling a response.
#[derive(Debug, Error)]
#[error("Error while dismantling response.")]
pub enum DismantleResponseError {
    /// Represents an error passed through from axum.
    #[error("Error from axum.")]
    AxumError(#[from] axum::Error),

    /// Represents an error to convert into UTF-8.
    #[error("Error while converting body bytes into string.")]
    FromUtf8Error(#[from] FromUtf8Error),
}

/// Represents an error occurring in middleware.
#[derive(Debug, Error)]
#[error("Error in middleware.")]
pub enum MiddlewareError {
    /// Represents an error while wrapping page.
    #[error("Error while wrapping page.")]
    WrapPage { source: DismantleResponseError },

    /// Represents an error while handling another error.
    #[error("This error occured while handling another error.")]
    HandleError { source: DismantleResponseError },
}

impl IntoResponse for MiddlewareError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

/// Represents an error while getting current page
#[derive(Debug, Error)]
#[error("Error while getting current page.")]
pub enum PageError {
    /// Represents an error while serving directory
    #[error("Error while serving directory.")]
    Dir(#[from] DirError),

    /// Represents an arbitrary I.O. error.
    #[error("I.O. error.")]
    IO(#[from] io::Error),

    /// Represents an error while reading from input.
    #[error(r#"Could not read "{}"."#, .path)]
    Read { path: String, source: io::Error },
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}

impl PageError {
    pub fn kind(&self) -> Option<io::ErrorKind> {
        match self {
            Self::IO(err) => Some(err.kind()),
            Self::Read { path: _, source } => Some(source.kind()),
            Self::Dir(err) => match err {
                DirError::IO(err) => Some(err.kind()),
                DirError::Read { path: _, source } => Some(source.kind()),
                _ => None,
            },
        }
    }
}

/// Represents an error while serving directory
#[derive(Debug, Error)]
#[error("Error while serving directory.")]
pub enum DirError {
    /// Represents an arbitrary I.O. error.
    #[error("I.O. error.")]
    IO(#[from] io::Error),

    /// Represents an error while reading from input.
    #[error(r#"Could not read "{}"."#, .path)]
    Read { path: String, source: io::Error },

    /// Represents an error relating to the manipulation of paths.
    Path(#[from] PathError),
}

/// Represents an error relating to the manipulation of paths
#[derive(Debug, Error)]
#[error("Path error.")]
pub enum PathError {
    /// Represents an error stripping prefix from path.
    #[error("Could not strip prefix.")]
    StripPrefix(#[from] StripPrefixError), // Maybe want to include prefix and path for printing?

    /// Represents an error getting file stem.
    #[error(r#"Could not get file stem of "{path}"."#)]
    FileStem { path: String },

    /// Represents an error converting the path to unicode.
    #[error(r#"Error while converting "{path}" to unicode."#)]
    Unicode { path: String },
}

#[derive(Debug, Error)]
#[error("Could not parse frontmatter.")]
pub enum FrontmatterError {
    // Represents an error parsing
    #[error("Could not find frontmatter.\n{0}")]
    Parse(String),

    /// Represents an error from de-serializing toml
    #[error("Error while de-serializing toml.")]
    TOMLError(#[from] toml::de::Error),
}

impl<'a> From<ParseError<'a>> for FrontmatterError {
    fn from(value: ParseError<'a>) -> Self {
        Self::Parse(format!("{value}"))
    }
}
