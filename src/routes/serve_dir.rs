use std::path::Path;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::AppState;

pub async fn serve_dir<P>(state: &AppState, path: &P) -> Response
where
    P: AsRef<Path>,
{
    let fs_path = state.content_root.join(path);

    let placeholder_message = format!(
        "The method to show this page has not been implemented yet! {}",
        fs_path.display()
    );
    (StatusCode::NOT_IMPLEMENTED, placeholder_message).into_response()
}
