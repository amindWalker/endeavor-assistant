use api_shared::prelude::LibError;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

use self::users::post_users_route;

pub mod users;

pub async fn run_server() -> Result<(), LibError> {
    let app = Router::new()
        .route("/", get(default_path))
        .route("/signin", post(post_users_route));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn default_path() -> Html<&'static str> {
    Html(
        "
    <div style='
        align='center'
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
