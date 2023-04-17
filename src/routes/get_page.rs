use std::path::Path;

use axum::{http::StatusCode, response::Response};

use crate::AppState;

use super::{serve_dir::serve_dir, serve_page::serve_page};

pub async fn get_page<P>(state: &AppState, path: P) -> Response
where
    P: AsRef<Path>,
{
    let response = serve_page(&state, &path).await;

    // Not found as a file, maybe is a directory?
    if response.status() != StatusCode::NOT_FOUND {
        return response;
    }

    serve_dir(&state, &path).await
}
