use crate::core::repo::connect_database;
use crate::server::start_server;
use config::get_config;
use tracing_subscriber::EnvFilter;

mod config;
mod core;
mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = get_config();
    let database = connect_database(&config.database_url);

    start_server(&config.endpoint_url, database).await;
}
