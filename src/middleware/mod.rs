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

#[derive(Debug, Template, Serialize)]
#[template(path = "error_page.html")]
struct ErrorPage<'a, T>
where
    T: std::fmt::Debug,
{
    status_code: &'a str,
    body: T,
}

pub async fn wrap_page<B>(req: Request<B>, next: Next<B>) -> Response {
    // Get response
    let response = next.run(req).await;
    // If not a success, don't wrap
    if !response.status().is_success() {
        return response;
    }

    // Extract body
    let (mut parts, body) = response.into_parts();
    // Remove content-length header
    parts.headers.remove("content-length");

    // String::from("Error in converting response body into bytes."),
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(ok) => ok.to_vec(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error in converting response body into bytes.",
            )
                .into_response();
        }
    };

    // String::from("Error converting request body to string: not valid UTF-8."),
    let content = match String::from_utf8(bytes) {
        Ok(ok) => ok,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error converting request body to stirng; not valid UTF-8.",
            )
                .into_response();
        }
    };

    let page = Page {
        title: "Page Title",
        content: content.as_str(),
    };

    Response::from_parts(parts, page.render().unwrap()).into_response()
}

pub async fn handle_error<B>(req: Request<B>, next: Next<B>) -> Response {
    // Get response
    let response = next.run(req).await;
    // // If success, don't care
    if response.status().is_success() {
        return response;
    }

    // Extract body
    let (mut parts, body) = response.into_parts();
    // Remove content-length header
    parts.headers.remove("content-length");
    parts
        .headers
        .append("content-type", "text/html; charset=utf-8".parse().unwrap());
    let status = parts.status;

    let error_page = ErrorPage {
        status_code: status.as_str(),
        body,
    };

    Response::from_parts(parts, error_page.render().unwrap()).into_response()
}
