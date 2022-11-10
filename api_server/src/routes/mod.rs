use axum::{
    response::Response,
    routing::{get, post},
    Router,
};

pub mod ping;
pub use ping::ping_path;
pub mod req;
pub use req::req_body;

pub async fn run() {
    let app = Router::new()
        .route("/", get(ping_path))
        .route("/tasks", get(req_body));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
