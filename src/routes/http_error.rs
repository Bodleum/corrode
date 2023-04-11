use askama::Template;
use axum::{
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
};

#[derive(Debug, Template)]
#[template(path = "error_page.html")]
struct HttpErrorPage {
    status_code: StatusCode,
    message: String,
}

pub async fn http_error_page<T: std::fmt::Debug>(req: Request<T>) -> impl IntoResponse {
    let http_error_page = HttpErrorPage {
        status_code: StatusCode::NOT_FOUND,
        message: format!("Page not found.\n{:#?} is an invalid request.", req),
    };

    Html(http_error_page.render().unwrap())
}
