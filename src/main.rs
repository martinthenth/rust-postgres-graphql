use crate::core::repo::connect_database;
use crate::server::build_schema;
use crate::server::start_server;
use clap::Parser;
use config::get_config;
use tracing_subscriber::EnvFilter;

mod config;
mod core;
mod server;
mod test;

/// RPG is a "Rust + Postgres + GraphQL example"
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Export the GraphQL SDL
    #[arg(short, long, default_value_t = false)]
    export: bool,
    /// Start the web server
    #[arg(short, long, default_value_t = false)]
    server: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.export {
        export_server_gql();
    }
    if args.server {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let config = get_config();
        let database = connect_database(&config.database_url);

        start_server(&config.endpoint_url, database).await;
    }
}

fn export_server_gql() {
    std::fs::write("docs/server.gql", &build_schema().sdl()).unwrap();
}
