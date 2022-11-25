use api_shared::prelude::LibError;
use axum::{
    http::Method,
    response::Html,
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

    const HOST: &str = "locahost";
    const PORT: &str = "3000";
    let listen_host_port = format!("{HOST}:{PORT}");
    axum::Server::bind(&listen_host_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn default_path() -> Html<&'static str> {
    Html(
        "
    <div style='
        align: center;
        border: 1px dashed red;
        border-radius: 16px;
        padding: 16px;
        height: 100vh;
    '>
        <h1 style='color: forestgreen;'>Do It Manager</h1>
        <p>{tasks}</p>
    </div>",
    )
}
