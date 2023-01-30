use api_shared::prelude::LibError;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use self::users::post_users_route;

pub mod users;

pub async fn run_server() -> Result<(), LibError> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(default_path))
        .route("/signin", post(post_users_route))
        .layer(cors);

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "3030";
    let listen_host_port = format!("{HOST}:{PORT}");

    println!("\n\n[::] Server running on http://{HOST}:{PORT}");
    axum::Server::bind(&listen_host_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn default_path() -> &'static str {
    "Home"
}
