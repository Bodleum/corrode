use askama::Template;
use axum::{
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::header;
use serde::Serialize;

#[derive(Debug, Template, Serialize)]
#[template(path = "page.html")]
struct Page<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Debug, Template, Serialize)]
#[template(path = "error_page.html")]
struct ErrorPage<T>
where
    T: std::fmt::Debug,
{
    status_code: String,
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
    parts.headers.remove(header::CONTENT_LENGTH);

    // TODO: Get page metadata

    // Get body
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

    // Set to html type
    parts.headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
    );

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
    parts.headers.remove(header::CONTENT_LENGTH);
    parts.headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
    );

    let error_page = ErrorPage {
        status_code: format!("{}", parts.status),
        body,
    };

    Response::from_parts(parts, error_page.render().unwrap()).into_response()
}
