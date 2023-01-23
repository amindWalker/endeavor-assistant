use api_shared::prelude::LibError;
use axum::{
    http::Method,
    response::Html,
    routing::{get, post},
    Router,
};
use dioxus::prelude::*;
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

    const HOST: &str = "0.0.0.0";
    const PORT: &str = "3000";
    let listen_host_port = format!("{HOST}:{PORT}");
    axum::Server::bind(&listen_host_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn default_path() -> Html<&'static str> {
    let mut state = use_state(&cx, || true);

    let app = cx.render(rsx!(
        main {
            class: "w-full h-full",
            "bg": if *state.get() { "black" } else { "white" },
            "text": "white",
            "hello there",
            span {
                "state: {state}"
            }
            button {
                onclick: move |_| {
                    state.set(!state);
                },
                "toggle"
            }
        }
    ));
    dioxus_web::launch(app);
    // Html(
    //     "
    // <div style='
    //     align: center;
    //     border: 1px dashed red;
    //     border-radius: 16px;
    //     padding: 16px;
    //     height: 100vh;
    // '>
    //     <h1 style='color: forestgreen;'>Do It Manager</h1>
    //     <p>{tasks}</p>
    // </div>",
    // )
}
