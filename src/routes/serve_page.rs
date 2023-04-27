use std::path::Path;

use axum::response::{Html, IntoResponse, Response};

use crate::{error::PageError, AppState};

pub async fn serve_page<P>(state: &AppState, path: &P) -> Result<Response, PageError>
where
    P: AsRef<Path> + ?Sized,
{
    let mut fs_path = state.content_root.join(path);
    fs_path.set_extension("md");

    let body = tokio::fs::read_to_string(&fs_path)
        .await
        .map_err(|source| PageError::Read {
            path: path.as_ref().display().to_string(),
            source,
        })?;
    Ok(Html(body).into_response())
}
