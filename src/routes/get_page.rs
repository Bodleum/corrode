use std::{io::ErrorKind, path::Path};

use axum::response::Response;
use error_stack::{Result, ResultExt};

use crate::{error::PageError, AppState};

use super::{serve_dir::serve_dir, serve_page::serve_page};

pub async fn get_page<P>(state: &AppState, path: &P) -> Result<Response, PageError>
where
    P: AsRef<Path> + ?Sized,
{
    let response = match serve_page(&state, &path).await {
        Ok(ok) => return Ok(ok),
        Err(err) => err,
    };

    // Not found as a file, maybe is a directory?
    if response.current_context().kind() != ErrorKind::NotFound {
        return Err(response.change_context(PageError).attach_printable(format!(
            "Error while serving page at {}.",
            &path.as_ref().display()
        )));
    }

    serve_dir(&state, &path)
        .await
        .change_context(PageError)
        .attach_printable(format!(
            "Error while serving page at {}.",
            &path.as_ref().display()
        ))
}
