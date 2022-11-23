use api_shared::prelude::LibError;

pub mod routes;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), LibError> {
    routes::run_server().await?;

    Ok(())
}
