use std::path::Path;

use axum::response::{Html, IntoResponse, Response};

pub async fn serve_page<P>(path: &P) -> Response
where
    P: AsRef<Path>,
{
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content).into_response()
}
