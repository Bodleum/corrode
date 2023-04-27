pub mod error;
mod middleware;
pub mod parsing;
mod routes;
mod web;

use std::path::PathBuf;

use routes::create_routes;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub content_root: PathBuf,
}

pub async fn run(app_state: AppState) {
    let app = create_routes().with_state(app_state);

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());
    let port = server.local_addr().port();
    println!("Listening on http://localhost:{port}");
    server.await.unwrap();
}

// pub async fn build
