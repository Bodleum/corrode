use std::path::Path;

use axum::response::{Html, IntoResponse, Response};
use hyper::StatusCode;

use crate::AppState;

pub async fn serve_page<P>(state: &AppState, path: &P) -> Response
where
    P: AsRef<Path>,
{
    let mut fs_path = state.content_root.join(&path);
    fs_path.set_extension("md");

    match tokio::fs::read_to_string(fs_path).await {
        Ok(ok) => Html(ok).into_response(),
        Err(err) => (
            StatusCode::NOT_FOUND,
            format!("Error reading {}. {}.", path.as_ref().display(), err),
        )
            .into_response(),
    }
}
