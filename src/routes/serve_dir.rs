use std::path::{Path, PathBuf};

use axum::response::{Html, IntoResponse, Response};
use error_stack::{IntoReport, Result, ResultExt};
use tokio::fs;

use crate::{
    error::{DirError, IOError, PathError},
    AppState,
};

pub async fn serve_dir<P>(state: &AppState, path: &P) -> Result<Response, DirError>
where
    P: AsRef<Path>,
{
    let fs_path = state.content_root.join(path);

    let contents = rel_contens(&fs_path)
        .await
        .change_context(DirError)
        .attach_printable(format!(
            "Error while getting contents of \"{}\".",
            path.as_ref().display()
        ))?;

    let links = format_links(&state, &contents)
        .change_context(DirError)
        .attach_printable(format!(
            "Error while formatting links in \"{}\".",
            path.as_ref().display()
        ))
        .attach_printable(format!("Links: {:#?}", contents))?;

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

async fn rel_contens<P>(path: &P) -> Result<Vec<PathBuf>, IOError>
where
    P: AsRef<Path>,
{
    let mut ret = Vec::new();
    let mut reader = fs::read_dir(path)
        .await
        .map_err(|err| IOError(err))
        .into_report()
        .attach_printable(format!(
            "Error while reading \"{}\".",
            path.as_ref().display()
        ))?;
    while let Some(entry) = reader
        .next_entry()
        .await
        .map_err(|err| IOError(err))
        .into_report()
        .attach_printable(format!(
            "Error getting next entry in \"{}\".",
            path.as_ref().display()
        ))?
    {
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
            to_url(&state, &p)?.display()
        ));
    }

    Ok(ret)
}

fn to_url<P>(state: &AppState, path: &P) -> Result<PathBuf, PathError>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .strip_prefix(&state.content_root)
        .map(|p| p.to_path_buf())
        .map_err(|err| PathError::StripPrefixError(err))
        .into_report()
        .attach_printable(format!(
            "Could not strip prefix \"{}\" from \"{}\".",
            "content",
            path.as_ref().display()
        ))
}

fn get_stem<P>(path: &P) -> Result<&str, PathError>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .file_stem()
        .ok_or(PathError::FileNameError(String::from(
            "Could not get filename!",
        )))
        .into_report()
        .attach_printable(format!(
            "Error while getting file stem of \"{}\".",
            path.as_ref().display()
        ))?
        .to_str()
        .ok_or(PathError::UnicodeError(String::from(
            "Could not convert filename to string, not valid Unicode!",
        )))
        .into_report()
        .attach_printable(format!(
            "Error while converting \"{}\" to string.",
            path.as_ref().display()
        ))
}
