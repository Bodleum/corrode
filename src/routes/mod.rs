mod get_page;
mod serve_dir;
mod serve_page;

use crate::{
    middleware::{handle_error, wrap_page},
    AppState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use get_page::get_page;

#[derive(Clone)]
pub struct PageHeader(pub String);

pub fn create_routes() -> Router<AppState, Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/", get(get_page_wrapper_main_page))
        .route("/*path", get(get_page_wrapper))
        .layer(axum::middleware::from_fn(wrap_page))
        .layer(axum::middleware::from_fn(handle_error))
        .route("/css/:file", get(get_css))
        .layer(cors)
}

async fn get_page_wrapper_main_page(state: State<AppState>) -> impl IntoResponse {
    get_page(&state, "").await
}

async fn get_page_wrapper(state: State<AppState>, Path(path): Path<String>) -> impl IntoResponse {
    get_page(&state, path).await
}

async fn get_css(Path(file): Path<String>) -> Response {
    let css = match tokio::fs::read_to_string(format!("assets/{file}")).await {
        Ok(ok) => ok,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found.").into_response(),
    };

    (
        StatusCode::OK,
        [("content-type", "text/css; charset=utf-8")],
        css,
    )
        .into_response()
}
