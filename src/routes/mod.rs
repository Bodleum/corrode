use crate::middleware::wrap_page;
use axum::{
    body::Body,
    extract::Path,
    http::{Method, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct PageHeader(pub String);

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/", get(index))
        .layer(axum::middleware::from_fn(wrap_page))
        .route("/css/:file", get(get_css))
        .layer(cors)
}

async fn index<B>(_req: Request<B>) -> impl IntoResponse
where
    B: std::fmt::Debug,
{
    Html("<h1>Peter!</h1>")
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
