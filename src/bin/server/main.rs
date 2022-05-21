#![feature(async_closure)]

use axum::extract::Path;
use tokio::{try_join};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::storage::storage;


mod storage;
mod test_cdn;

#[tokio::main]
async fn main() {
    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "test_cdn")]
        test_cdn::test_cdn().await
    });
    let server = tokio::task::spawn( async move {server().await});
    try_join!(test_cdn,server).unwrap();
}



async fn server() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = storage()
        .route("/",axum::routing::get( async move ||{
            storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
        }))
        .fallback(
            axum::routing::get( async move ||{
                storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
            }))
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}