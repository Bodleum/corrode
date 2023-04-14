use std::path::Path;

use axum::response::{Html, IntoResponse, Response};

pub async fn serve_dir<P>(path: &P) -> Response
where
    P: AsRef<Path>,
{
    Html(format!(
        " ### Info
Path: {} (Directory)",
        path.as_ref().display(),
    ))
    .into_response()
}
