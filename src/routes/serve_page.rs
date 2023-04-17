use std::path::Path;

use axum::response::{Html, IntoResponse};
use hyper::StatusCode;

use crate::AppState;

pub async fn serve_page<P>(state: &AppState, path: &P) -> impl IntoResponse
where
    P: AsRef<Path>,
{
    let mut fs_path = state.content_root.join(&path);
    fs_path.set_extension("md");

    match tokio::fs::read_to_string(fs_path).await {
        Ok(ok) => Ok(Html(ok)),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            format!("Error reading {}. {}.", path.as_ref().display(), err),
        )),
    }
}
