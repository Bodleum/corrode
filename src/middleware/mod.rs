use askama::Template;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse},
};
use serde::Serialize;

#[derive(Debug, Template, Serialize)]
#[template(path = "page.html")]
struct Page {
    title: String,
    content: String,
}

pub async fn wrap_page<B>(req: Request<B>, next: Next<B>) -> Result<impl IntoResponse, StatusCode> {
    // Get response
    let response = next.run(req).await;

    // Extract body
    let (_parts, body) = response.into_parts();
    let content = String::from_utf8(
        hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_vec(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let page = Page {
        title: String::from("Page Title"),
        content,
    };

    Ok(Html(page.render().unwrap()))
}
