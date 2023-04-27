use std::{io::ErrorKind, path::Path};

use axum::response::Response;

use super::{serve_dir::serve_dir, serve_page::serve_page};
use crate::{error::PageError, AppState};

pub async fn get_page<P>(state: &AppState, path: &P) -> Result<Response, PageError>
where
    P: AsRef<Path> + ?Sized,
{
    let response = match serve_page(state, path).await {
        Ok(ok) => return Ok(ok),
        Err(err) => err,
    };

    // Not found as a file, maybe is a directory?
    if let Some(ErrorKind::NotFound) = response.kind() {
        serve_dir(state, path).await.map_err(|err| err.into())

    // Any other error we pass upwards
    } else {
        Err(response)
    }
}
