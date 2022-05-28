#![feature(async_closure)]

use std::sync::Arc;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Path;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::routing::post;
use axum::{Extension, Router};
use axum::body::Body;
use axum::response::Html;
use lazy_static::lazy_static;
use sea_orm::{DatabaseConnection, Schema};

use tokio::{try_join};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::storage::storage;
use migration::{Migrator, MigratorTrait};
use crate::graphql::{PoetShuffleSchema, populate_db_with_test_data};


mod storage;
mod local_cdn;
mod graphql;

lazy_static!{
    pub static ref DATABASE_URL: String ={
        #[cfg(feature="dev")]
        return dotenv_codegen::dotenv!("DATABASE_URL").to_string();
        #[cfg(not(feature="dev"))]
        std::env::var("DATABASE_URL").unwrap()
    };
    pub static ref JWT_SECRET: String = {
        #[cfg(feature="dev")]
        return dotenv_codegen::dotenv!("JWT_SECRET").to_string();
        #[cfg(not(feature="dev"))]
        std::env::var("JWT_SECRET").unwrap()
    };

}



#[tokio::main]
async fn main() {
    // Load DB url from env vars and make sure we are up on latest
    // migration.
    let connection = sea_orm::Database::connect(&*DATABASE_URL).await
        .expect("Expecting DB connection given DATABASE_URL.");
    Migrator::up(&connection, None).await
        .expect("Expecting Successful migration.");
    // ...
    #[cfg(feature="dev")]
    populate_db_with_test_data(&connection).await.unwrap();
    // Spawn test_cdn server on port 8001 during development.
    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "local_cdn")]
        local_cdn::local_cdn().await
    });
    // Spawn our normal HTTP server to handle API calls.
    let server = tokio::task::spawn( async move {server(connection).await});
    // We run all processes until the first error.
    try_join!(test_cdn,server).unwrap();
}

/// Executes GraphQL requests against out schema stored in extensions.
async fn graphql_handler(
    schema: Extension<PoetShuffleSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// builds our HTTP server, needs DB conn for GraphQL.
async fn server(conn:DatabaseConnection) {
    // Normal tracing boilerplate to get traces, see tracing docs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
        let api_routes = Router::new()
        .route("/graphql",post(graphql_handler));
    // For use during development.
    #[cfg(feature = "graphiql")]
       let api_routes =  api_routes
        .route("/graphiql",axum::routing::any(async move ||{
            Html(
            async_graphql::http::graphiql_source("/api/graphql",None)
            )
        }));
    // Using storage() as a base which handles arbitrary file lookups.
    let app = storage()
        .route("/",axum::routing::get( async move ||{
            storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
        }))
        .nest("/api",api_routes)
        // For our SPA to properly function we need to respond to non-supported
        // urls with the SPA itself.
        .fallback(
            axum::routing::get( async move ||{
                storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
            }))
        // Add tracing to our service.
        .layer(TraceLayer::new_for_http())
        // Add our Graphql schema for our handler.
        .layer(Extension(graphql::new_schema(conn)));
    // See Axum docs for standard server boilerplate.
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}