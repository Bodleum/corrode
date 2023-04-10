mod middleware;
mod routes;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());
    let port = server.local_addr().port();
    println!("Listening on http://localhost:{port}");
    server.await.unwrap();
}
