use askama::Template;
use axum::{
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
};

pub struct Page(pub String);

#[derive(Debug, Template)]
#[template(path = "error_page.html")]
struct HttpErrorPage {
    status_code: StatusCode,
    message: String,
}

pub async fn error(status_code: StatusCode, message: String) -> impl IntoResponse {
    Html(
        HttpErrorPage {
            status_code,
            message,
        }
        .render()
        .unwrap(),
    )
    .into_response()
}

pub async fn error_resp<B: std::fmt::Debug>(resp: Response<B>) -> impl IntoResponse {
    error(
        resp.status(),
        format!("{:#?} is an invalid request.", resp.body()),
    )
    .await
}

pub async fn error_req<B: std::fmt::Debug>(
    req: Request<B>,
    status_code: StatusCode,
) -> impl IntoResponse {
    error(
        status_code,
        format!("{:#?} is an invalid request.", req.into_body()),
    )
    .await
}
