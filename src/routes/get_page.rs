use std::{io::ErrorKind, path::Path};

use axum::response::Response;

use crate::{error::ServeDirError, AppState};

use super::{serve_dir::serve_dir, serve_page::serve_page};

pub async fn get_page<P>(state: &AppState, path: &P) -> Result<Response, ServeDirError>
where
    P: AsRef<Path> + ?Sized,
{
    let response = match serve_page(&state, &path).await {
        Ok(ok) => return Ok(ok),
        Err(err) => err,
    };

    // Not found as a file, maybe is a directory?
    if response.0.kind() != ErrorKind::NotFound {
        return Err(response.into());
    }

    serve_dir(&state, &path).await
}
