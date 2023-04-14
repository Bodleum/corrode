use std::path::Path;

use axum::response::{Html, IntoResponse, Response};
use hyper::StatusCode;

pub async fn serve_page<P>(path: &P) -> Response
where
    P: AsRef<Path>,
{
    let content = match tokio::fs::read_to_string(path).await {
        Ok(ok) => ok,
        Err(err) => {
            return (
                StatusCode::NOT_FOUND,
                format!("Error reading {}. {}.", path.as_ref().display(), err),
            )
                .into_response()
        }
    };

    Html(content).into_response()
}
