use std::{io, path::Path};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tokio::fs::{self, File};

use crate::{error::IOError, AppState};

pub async fn serve_dir<P>(state: &AppState, path: &P) -> Result<Response, IOError>
where
    P: AsRef<Path>,
{
    let fs_path = state.content_root.join(path);

    let mut foo = Vec::new();
    for f in rel_contens(&fs_path).await? {
        foo.push(f);
    }

    let placeholder_message = format!(
        "The method to show this page has not been implemented yet! {}",
        fs_path.display()
    );

    Ok((StatusCode::NOT_IMPLEMENTED, placeholder_message).into_response())
}

async fn rel_contens<P>(path: &P) -> io::Result<Vec<File>>
where
    P: AsRef<Path>,
{
    let mut ret = Vec::new();
    let mut reader = fs::read_dir(path).await?;
    while let Some(entry) = reader.next_entry().await? {
        // Want only directories or markdown files
        if (entry.path().is_dir()) || (entry.path().extension().unwrap_or_default() == "md") {
            ret.push(File::open(entry.path()).await?);
        }
    }

    Ok(ret)
}
