#![deny(clippy::unwrap_used)]

use chrono::Utc;
use tracing::info;

mod delete;
mod executor;
mod generate;
mod migrate;
mod replace;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_time = Utc::now().time();

    tracing_subscriber::fmt::init();
    executor::run().await?;

    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    info!("Execution time: {} seconds", diff.num_seconds());
    Ok(())
}
