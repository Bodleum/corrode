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

    let mut foo = Vec::new();
    for f in rel_contens(&fs_path)
        .await
        .change_context(DirError)
        .attach_printable(format!(
            "Error while getting contents of \"{}\".",
            path.as_ref().display()
        ))?
    {
        foo.push(f);
    }

    let links = format_links(&foo)
        .change_context(DirError)
        .attach_printable(format!(
            "Error while formatting links in \"{}\".",
            path.as_ref().display()
        ))
        .attach_printable(format!("Links: {:#?}", foo))?;

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

fn format_links(files: &Vec<PathBuf>) -> Result<Vec<String>, PathError> {
    use convert_case::{Case, Casing};

    let mut ret = Vec::new();
    for p in files {
        let url_path = to_url(&p)?.display();
        let display = p
            .file_stem()
            .ok_or(PathError::FileNameError(String::from(
                "Could not get filename!",
            )))
            .into_report()
            .attach_printable(format!(
                "Error while getting file stem of \"{}\".",
                p.display()
            ))?
            .to_str()
            .ok_or(PathError::UnicodeError(String::from(
                "Could not convert filename to string, not valid Unicode!",
            )))
            .into_report()
            .attach_printable(format!(
                "Error while converting \"{}\" to string.",
                p.display()
            ))?
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
        .map_err(|err| PathError::StripPrefixError(err))
        .into_report()
        .attach_printable(format!(
            "Could not strip prefix \"{}\" from \"{}\".",
            "content",
            path.as_ref().display()
        ))
}
