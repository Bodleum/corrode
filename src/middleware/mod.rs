use askama::Template;
use axum::{
    http::{response::Parts, HeaderValue, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::header;
use serde::Serialize;

use crate::error::MiddlewareError;

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

async fn dismantle_response(resp: Response) -> Result<(Parts, String), MiddlewareError> {
    // Extract body and headers
    let (parts, box_body) = resp.into_parts();
    // Get body
    let bytes = hyper::body::to_bytes(box_body).await?.to_vec();
    let body = String::from_utf8(bytes)?;
    Ok((parts, body))
}

pub async fn wrap_page<B>(req: Request<B>, next: Next<B>) -> Result<Response, MiddlewareError> {
    // Get response
    let response = next.run(req).await;
    // If not a success, don't wrap
    if !response.status().is_success() {
        return Ok(response);
    }

    let (mut parts, body) = dismantle_response(response).await?;
    // Remove content-length header
    parts.headers.remove(header::CONTENT_LENGTH);

    // TODO: Get page metadata
    let page = Page {
        title: "Page Title",
        content: body.as_str(),
    };

    // Set to html type
    parts.headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
    );

    Ok(Response::from_parts(parts, page.render().unwrap()).into_response())
}

pub async fn handle_error<B>(req: Request<B>, next: Next<B>) -> Result<Response, MiddlewareError> {
    // Get response
    let response = next.run(req).await;
    // // If success, don't care
    if response.status().is_success() {
        return Ok(response);
    }

    let (mut parts, body) = dismantle_response(response).await?;
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

    Ok(Response::from_parts(parts, error_page.render().unwrap()).into_response())
}
