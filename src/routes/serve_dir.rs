use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use axum::{
    http::{HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
};
use hyper::header;

pub async fn serve_dir<P>(path: &P) -> Response
where
    P: AsRef<Path>,
{
    // TODO: make this better.
    //       Pass in fs_path and url path as view into it
    let url_path = match path.as_ref().strip_prefix("content") {
        Ok(ok) => ok,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };

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

    let basenames = match paths_to_basenames(&contents).await {
        Ok(ok) => ok,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    };

    let links = basenames
        .iter()
        .map(|bn| {
            let (a, b) = format_basename(bn);
            format!(" - [{}]({}/{})", a, url_path.display(), b)
        })
        .collect::<Vec<_>>()
        .join("\n");

    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
        )],
        format!(
            " ### Info
Path: {} (Directory)

### Contents
{}",
            path.as_ref().display(),
            links,
        ),
    )
        .into_response()
}

// TODO: Don't want only markdown files, want directories as well!
async fn get_markdown_files_in<P>(path: &P) -> std::io::Result<HashSet<PathBuf>>
where
    P: AsRef<Path>,
{
    type T = std::io::Result<Vec<std::fs::DirEntry>>;
    Ok(path
        .as_ref()
        .read_dir()?
        .collect::<T>()?
        .iter()
        // Keep only if extension is 'md'
        .filter_map(|entry| (entry.path().extension()? == "md").then_some(entry.path()))
        .collect::<HashSet<PathBuf>>())
}

// TODO: Change so function operates on a single item, then can return the item which caused the
//       error
async fn paths_to_basenames<P>(paths: &HashSet<P>) -> Result<HashSet<&str>, &'static str>
where
    P: AsRef<Path>,
{
    Ok(paths
        .iter()
        // Get filename
        .map(|p| p.as_ref().file_name())
        .collect::<Option<HashSet<_>>>()
        .ok_or("Could not get filename")?
        // Convert to &str
        .iter()
        .map(|s| s.to_str())
        .collect::<Option<HashSet<_>>>()
        .ok_or("Could not read filename: not valid Unicode!")?
        // Split at '.'
        .iter()
        .map(|str| str.split_once('.'))
        .collect::<Option<HashSet<_>>>()
        .ok_or("Could not get basename of file: no '.' found!")?
        // Keep only first part
        .iter()
        .map(|i| i.0)
        .collect::<HashSet<_>>())
}

fn format_basename<T>(bn: &T) -> (String, &T)
where
    T: AsRef<str> + convert_case::Casing<T>,
{
    use convert_case::Case;

    (bn.to_case(Case::Title), bn)
}
