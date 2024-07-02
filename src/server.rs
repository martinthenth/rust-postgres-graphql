use crate::server::schema::create_schema;
use crate::server::schema::GraphSchema;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Json;
use axum::routing::get;
use axum::routing::post;
use axum::serve;
use axum::Router;
use deadpool_diesel::postgres::Pool;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod resolvers;
mod schema;

/// Start the web server
pub async fn start_server(endpoint_url: &String, database: Pool) {
    let schema = create_schema(database);

    // TODO: Add tool to export GQL and SQL
    std::fs::write("docs/server.gql", &schema.sdl()).unwrap();

    let server = Router::new()
        .route("/", get(graphql_html))
        .route("/graph", post(graphql_json))
        .fallback(fallback_json)
        .with_state(schema);
    let address: SocketAddr = endpoint_url.parse().unwrap();
    let listener = TcpListener::bind(&address).await.unwrap();

    info!("Running endpoint at {} (http)", address);
    serve(listener, server).await.unwrap();
}

/// Render the GraphiQL Playground HTML.
async fn graphql_html() -> (StatusCode, Html<String>) {
    (
        StatusCode::OK,
        Html(GraphiQLSource::build().endpoint("/graph").finish()),
    )
}

/// Render the GraphQL JSON.
async fn graphql_json(state: State<GraphSchema>, req: GraphQLRequest) -> GraphQLResponse {
    state.execute(req.into_inner()).await.into()
}

/// Render the fallback JSON.
async fn fallback_json() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "status": "Not Found" })),
    )
}
