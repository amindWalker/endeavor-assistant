use axum::{
    routing::{get, post},
    Router, response::Html,
};

pub async fn run() {
    let app = Router::new()
        .route("/", get(ping))
        .route("/tasks", post(req_body));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> Html<&'static str> {
    Html("
    <div style='
        border: 1px dashed red;
        border-radius: 16px;
        padding: 16px;
        height: 100vh;
    '>
        <h1 style='color: forestgreen;'>Pong!</h1>
    </div>")
}

async fn req_body(body: String) -> String {
    body
}