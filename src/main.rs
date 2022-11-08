use do_it::run;

mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> crate::prelude::Result<()> {
    run().await;

    Ok(())
}