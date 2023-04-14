use askama::Template;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Template, Serialize)]
#[template(path = "page.html")]
struct Page<'a> {
    title: &'a str,
    content: &'a str,
}

pub async fn wrap_page<B: std::fmt::Debug>(
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get response
    let response = next.run(req).await;

    // Extract body
    let (mut parts, body) = response.into_parts();
    // Remove content-length header
    parts.headers.remove("content-length");
    // Propagate status code if not a success
    if !parts.status.is_success() {
        return Err(parts.status);
    }

    // String::from("Error in converting response body into bytes."),
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_vec();

    // String::from("Error converting request body to string: not valid UTF-8."),
    let content = String::from_utf8(bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let page = Page {
        title: "Page Title",
        content: content.as_str(),
    };

    Ok(Response::from_parts(parts, page.render().unwrap()))
}
