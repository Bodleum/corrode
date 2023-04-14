mod get_page;
mod serve_page;

use crate::middleware::wrap_page;
use axum::{
    body::Body,
    extract::Path,
    http::{Method, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use get_page::get_page;

#[derive(Clone)]
pub struct PageHeader(pub String);

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/", get(get_page_wrapper_main_page))
        .route("/*path", get(get_page_wrapper))
        .layer(axum::middleware::from_fn(wrap_page))
        .route("/css/:file", get(get_css))
        .fallback(serve_404)
        .layer(cors)
}

async fn get_page_wrapper_main_page<B>(_req: Request<B>) -> impl IntoResponse {
    get_page("").await
}

async fn get_page_wrapper(Path(path): Path<String>) -> impl IntoResponse {
    get_page(path).await
}

async fn get_css(Path(file): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let css = tokio::fs::read_to_string(format!("assets/{file}"))
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Response::builder()
        .header("content-type", "text/css; charset=utf-8")
        .body(css)
        .unwrap())
}

async fn serve_404<B: std::fmt::Debug>(req: Request<B>) -> impl IntoResponse {
    crate::web::error_resp(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(req.into_body())
            .unwrap(),
    )
    .await
}
