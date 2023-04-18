use std::path::Path;

use axum::response::{Html, IntoResponse, Response};
use error_stack::{IntoReport, Result, ResultExt};

use crate::{error::IOError, AppState};

pub async fn serve_page<P>(state: &AppState, path: &P) -> Result<Response, IOError>
where
    P: AsRef<Path>,
{
    let mut fs_path = state.content_root.join(&path);
    fs_path.set_extension("md");

    let body = tokio::fs::read_to_string(&fs_path)
        .await
        .map_err(|err| IOError(err))
        .into_report()
        .attach_printable(format!("Could not read {}.", &fs_path.display()))?;
    Ok(Html(body).into_response())
}
