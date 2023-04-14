use std::path::Path;

use axum::response::Html;

pub async fn serve_dir<P>(path: &P) -> Html<String>
where
    P: AsRef<Path>,
{
    Html(format!(
        " ### Info
Path: {} (Directory)",
        path.as_ref().display(),
    ))
}
