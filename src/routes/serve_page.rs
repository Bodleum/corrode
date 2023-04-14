use std::path::Path;

use axum::response::Html;

pub async fn serve_page<P>(path: &P) -> Html<String>
where
    P: AsRef<Path>,
{
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
