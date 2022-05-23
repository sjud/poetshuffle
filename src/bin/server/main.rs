#![feature(async_closure)]

use std::sync::Arc;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Router};

use tokio::{try_join};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::storage::storage;
use migration::{Migrator, MigratorTrait};


mod storage;
mod test_cdn;
mod graphql;

const DATABASE_URL :&str = dotenv_codegen::dotenv!("DATABASE_URL");
const JWT_SECRET: &str = dotenv_codegen::dotenv!("JWT_SECRET");


#[tokio::main]
async fn main() {
    // Load DB url from env vars and make sure we are up on latest
    // migration.
    let connection = sea_orm::Database::connect(DATABASE_URL).await
        .expect("Expecting DB connection given DATABASE_URL.");
    Migrator::up(&connection, None).await
        .expect("Expecting Successful migration.");
    // Spawn test_cdn server on port 8001 during development.
    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "test_cdn")]
        test_cdn::test_cdn().await
    });
    // Spawn our normal HTTP server to handle API calls.
    let server = tokio::task::spawn( async move {server().await});
    // We run all processes until the first error.
    try_join!(test_cdn,server).unwrap();
}




async fn server() {
    // Normal tracing boilerplate to get traces, see tracing docs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Using storage() as a base which handles arbitrary file lookups.
    let app = storage()
        .route("/",axum::routing::get( async move ||{
            storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
        }))
        // For our SPA to properly function we need to respond to non-supported
        // urls with the SPA itself.
        .fallback(
            axum::routing::get( async move ||{
                storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
            }))
        .layer(TraceLayer::new_for_http());
    // See Axum docs for standard server boilerplate.
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}