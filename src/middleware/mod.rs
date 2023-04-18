use askama::Template;
use axum::{
    http::{response::Parts, HeaderValue, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use error_stack::{IntoReport, ResultExt};
use hyper::header;
use serde::Serialize;

use crate::error::{AxumError, FromUTF8Error, MiddlewareError, MiddlewareReport};

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
    T: std::fmt::Display,
{
    status_code: String,
    body: T,
}

async fn dismantle_response(
    resp: Response,
) -> error_stack::Result<(Parts, String), MiddlewareError> {
    // Extract body and headers
    let (parts, box_body) = resp.into_parts();
    // Get body
    let bytes = hyper::body::to_bytes(box_body)
        .await
        .map_err(|err| AxumError(err))
        .into_report()
        .change_context(MiddlewareError)
        .attach_printable("Error while converting response body into bytes.")?
        .to_vec();
    let body = String::from_utf8(bytes)
        .map_err(|err| FromUTF8Error(err))
        .into_report()
        .change_context(MiddlewareError)
        .attach_printable("Error while converting body bytes into string.")?;

    Ok((parts, body))
}

pub async fn wrap_page<B>(req: Request<B>, next: Next<B>) -> Result<Response, MiddlewareReport> {
    // Get response
    let response = next.run(req).await;
    // If not a success, don't wrap
    if !response.status().is_success() {
        return Ok(response);
    }

    let (mut parts, body) = dismantle_response(response)
        .await
        .attach_printable("Error while wrapping page.")?;
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

pub async fn handle_error<B>(req: Request<B>, next: Next<B>) -> Result<Response, MiddlewareReport> {
    // Get response
    let response = next.run(req).await;
    // // If success, don't care
    if response.status().is_success() {
        return Ok(response);
    }

    let (mut parts, body) = dismantle_response(response)
        .await
        .attach_printable("This error occured while handling another error.")?;
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
