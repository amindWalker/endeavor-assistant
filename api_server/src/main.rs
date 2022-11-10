use routes::run;

mod error;
mod prelude;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> crate::prelude::Result<()> {
    run().await;

    Ok(())

}
