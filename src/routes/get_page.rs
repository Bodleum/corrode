use std::path::Path;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::AppState;

use super::{serve_dir::serve_dir, serve_page::serve_page};

pub async fn get_page<P>(state: &AppState, path: &P) -> Response
where
    P: AsRef<Path> + ?Sized,
{
    let response = serve_page(&state, &path).await.into_response();

    // Not found as a file, maybe is a directory?
    if response.status() != StatusCode::NOT_FOUND {
        return response;
    }

    serve_dir(&state, &path).await
}
