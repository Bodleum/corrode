use std::{
    io,
    path::{Path, PathBuf},
};

use axum::response::{Html, IntoResponse, Response};

use tokio::fs::{self, File};

use crate::{
    error::{PathError, ServeDirError},
    AppState,
};

pub async fn serve_dir<P>(state: &AppState, path: &P) -> Result<Response, ServeDirError>
where
    P: AsRef<Path>,
{
    dbg!(path.as_ref().display());
    let fs_path = state.content_root.join(path);

    let mut foo = Vec::new();
    for f in rel_contens(&fs_path).await? {
        foo.push(f);
    }

    let links = format_links(foo)?;

    let placeholder_message = format!(
        "# Not Implemented

{}

The method to show this page has not been implemented yet!

{}",
        fs_path.display(),
        links.join("\n"),
    );

    Ok(Html(placeholder_message).into_response())
}

async fn rel_contens<P>(path: &P) -> io::Result<Vec<(File, PathBuf)>>
where
    P: AsRef<Path>,
{
    let mut ret = Vec::new();
    let mut reader = fs::read_dir(path).await?;
    while let Some(entry) = reader.next_entry().await? {
        // Want only directories or markdown files
        if (entry.path().is_dir()) || (entry.path().extension().unwrap_or_default() == "md") {
            ret.push((File::open(entry.path()).await?, entry.path()));
        }
    }

    Ok(ret)
}

fn format_links(files: Vec<(File, PathBuf)>) -> Result<Vec<String>, PathError> {
    use convert_case::{Case, Casing};

    let mut ret = Vec::new();
    for (_, p) in files {
        let url_path = to_url(&p)?.display();
        let display = p
            .file_stem()
            .ok_or(PathError::Custom(String::from("Could not get filename!")))?
            .to_str()
            .unwrap()
            .to_case(Case::Title);
        ret.push(format!(" - [{}]({})", display, url_path));
    }

    Ok(ret)
}

fn to_url<P>(path: &P) -> Result<&Path, PathError>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .strip_prefix("content")
        .map_err(|err| err.into())
}
