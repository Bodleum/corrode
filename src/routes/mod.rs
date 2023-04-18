mod get_page;
mod serve_dir;
mod serve_page;

use crate::{
    error::PageReport,
    middleware::{handle_error, wrap_page},
    AppState,
};
use axum::{
    body::{Body, StreamBody},
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio_util::io::ReaderStream;
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
        .route("/font/:file", get(get_font))
        .route("/css/:file", get(get_css))
        .route("/favicon.ico", get(|| async { StatusCode::NOT_FOUND }))
        .layer(cors)
}

async fn get_page_wrapper_main_page(state: State<AppState>) -> Result<Response, PageReport> {
    get_page(&state, "").await.map_err(|err| err.into())
}

async fn get_page_wrapper(
    state: State<AppState>,
    Path(path): Path<String>,
) -> Result<Response, PageReport> {
    get_page(&state, &path).await.map_err(|err| err.into())
}

async fn get_font(Path(font): Path<String>) -> impl IntoResponse {
    get_asset("font", font).await
}

async fn get_css(Path(css): Path<String>) -> impl IntoResponse {
    get_asset("css", css).await
}

async fn get_asset<T, U>(asset: T, file: U) -> impl IntoResponse
where
    T: AsRef<std::path::Path>,
    U: AsRef<std::path::Path>,
{
    let mut path = std::path::PathBuf::from("assets");
    path.push(asset);
    path.push(file);
    match tokio::fs::File::open(path).await {
        Ok(ok) => Ok(StreamBody::new(ReaderStream::new(ok))),
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    }
}
