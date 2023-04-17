use std::path::Path;

use axum::response::{Html, IntoResponse, Response};

use crate::{error::IOError, AppState};

pub async fn serve_page<P>(state: &AppState, path: &P) -> Result<Response, IOError>
where
    P: AsRef<Path>,
{
    let mut fs_path = state.content_root.join(&path);
    fs_path.set_extension("md");

    Ok(Html(tokio::fs::read_to_string(fs_path).await?).into_response())
}
