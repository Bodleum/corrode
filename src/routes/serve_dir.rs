use std::path::{Path, PathBuf};

use axum::response::{Html, IntoResponse, Response};
use tokio::fs;

use crate::{
    error::{DirError, PathError},
    AppState,
};

pub async fn serve_dir<P>(state: &AppState, path: &P) -> Result<Response, DirError>
where
    P: AsRef<Path> + ?Sized,
{
    let fs_path = state.content_root.join(path);

    let contents = rel_contens(&fs_path).await?;
    let links = format_links(state, &contents)?;

    let placeholder_message = format!(
        "# Not Implemented

{}

The method to show this page has not been implemented yet!

{}",
        path.as_ref().display(),
        links.join("\n"),
    );

    Ok(Html(placeholder_message).into_response())
}

async fn rel_contens<P>(path: &P) -> Result<Vec<PathBuf>, DirError>
where
    P: AsRef<Path>,
{
    let mut ret = Vec::new();
    let mut reader = fs::read_dir(path).await.map_err(|source| DirError::Read {
        path: path.as_ref().display().to_string(),
        source,
    })?;
    while let Some(entry) = reader.next_entry().await? {
        // Want only directories or markdown files
        if (entry.path().is_dir()) || (entry.path().extension().unwrap_or_default() == "md") {
            ret.push(entry.path());
        }
    }

    Ok(ret)
}

fn format_links(state: &AppState, files: &Vec<PathBuf>) -> Result<Vec<String>, PathError> {
    use convert_case::{Case, Casing};

    let mut ret = Vec::new();
    for p in files {
        let display = get_stem(&p)?.to_case(Case::Title);
        ret.push(format!(
            " - [{}](/{})",
            display,
            to_url(state, &p)?.display()
        ));
    }

    Ok(ret)
}

fn to_url<P>(state: &AppState, path: &P) -> Result<PathBuf, PathError>
where
    P: AsRef<Path>,
{
    Ok(path
        .as_ref()
        .strip_prefix(&state.content_root)?
        .to_path_buf())
}

fn get_stem<P>(path: &P) -> Result<&str, PathError>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .file_stem()
        .ok_or(PathError::FileStem {
            path: path.as_ref().display().to_string(),
        })?
        .to_str()
        .ok_or(PathError::Unicode {
            path: path.as_ref().display().to_string(),
        })
}
