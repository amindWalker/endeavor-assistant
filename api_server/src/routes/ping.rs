use axum::response::Html;

pub async fn ping_path() -> Html<&'static str> {
    Html(
        "
    <div style='
        border: 1px dashed red;
        border-radius: 16px;
        padding: 16px;
        height: 100vh;
    '>
        <h1 style='color: forestgreen;'>Pong!</h1>
    </div>",
    )
}
