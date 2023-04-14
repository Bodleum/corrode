use std::{
    collections::HashSet,
    fs::DirEntry,
    path::{Path, PathBuf},
};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub async fn serve_dir<P>(path: &P) -> Response
where
    P: AsRef<Path>,
{
    let contents = match get_markdown_files_in(path).await {
        Ok(ok) => ok,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading {}. {}", path.as_ref().display(), err),
            )
                .into_response()
        }
    };

    Html(format!(
        " ### Info
Path: {} (Directory)

### Contents
{:#?}",
        path.as_ref().display(),
        contents,
    ))
    .into_response()
}

async fn get_markdown_files_in<P>(path: &P) -> std::io::Result<HashSet<PathBuf>>
where
    P: AsRef<Path>,
{
    type T = std::io::Result<Vec<DirEntry>>;
    Ok(path
        .as_ref()
        .read_dir()?
        .collect::<T>()?
        .iter()
        // Keep only if extension is 'md'
        .filter_map(|entry| (entry.path().extension()? == "md").then_some(entry.path()))
        .collect::<HashSet<PathBuf>>())
}
