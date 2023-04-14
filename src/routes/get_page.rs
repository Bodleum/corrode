use std::path::{Path, PathBuf};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

// const CONTENT_ROOT: &'static str = "/home/bodleum/archive/Dev/Rust/corrode/content";
const CONTENT_ROOT: &'static str = "content";

enum TypeInFS {
    File,
    Dir,
    NotFound,
}

impl std::fmt::Display for TypeInFS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => f.write_str("File"),
            Self::Dir => f.write_str("Directory"),
            Self::NotFound => f.write_str("Not Found"),
        }
    }
}

pub async fn get_page<P>(path: P) -> Result<impl IntoResponse, StatusCode>
where
    P: AsRef<Path>,
{
    // Content to be served
    let mut content: String = String::new();

    // Get path and type in file-system
    let path = get_fs_path(path).await.ok_or(StatusCode::BAD_REQUEST)?;
    let ft = get_type(&path).await;

    // match ft {
    //     TypeInFS::File => {
    //         path.set_extension("md");
    //         return serve_page(path).await;
    //     }
    //     _ => {}
    // }

    content.push_str(
        format!(
            " ### Info
Path: {} ({})",
            path.display(),
            ft
        )
        .as_str(),
    );

    Ok(Html(content))
}

async fn get_fs_path<P>(path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let mut fs_path = PathBuf::from(CONTENT_ROOT);
    fs_path.push(path);

    if fs_path.is_dir() {
        return Some(fs_path);
    }

    fs_path.set_extension("md");
    if fs_path.is_file() {
        return Some(fs_path);
    }

    None
}

async fn get_type<P>(path: &P) -> TypeInFS
where
    P: AsRef<Path>,
{
    if path.as_ref().is_dir() {
        return TypeInFS::Dir;
    }
    if path.as_ref().is_file() {
        return TypeInFS::File;
    }
    return TypeInFS::NotFound;
}
