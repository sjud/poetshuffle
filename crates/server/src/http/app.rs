use axum::response::Html;
use axum::{Extension, Router};
use axum::routing::{get,post,put};
use hmac::Hmac;
use reqwest::header::HeaderValue;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::graphql::schema::PoetShuffleSchema;
use crate::http::handlers::{graphql_handler, health_check, index_html, presign_url};
use crate::http::presign_url::presign_url_as_string;
use crate::http::upload::{upload_file,delete_file, upload_ws};
use crate::storage::StorageApi;

pub fn app(key: Hmac<Sha256>, schema: PoetShuffleSchema, conn: DatabaseConnection) -> Router {
    // For whatever reason this is required for upload
    // despite our app being served from the server
    let api_routes = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/health_check",get(health_check))
        .route("/upload_ws/:jwt",get(upload_ws))
        .route("/upload_file",put(upload_file))
        .route("/delete_file",put(delete_file))
        .route("/presign_url",get(presign_url_as_string));

    // For use during development.
    #[cfg(feature = "graphiql")]
        let api_routes = api_routes.route(
        "/graphiql",
        axum::routing::any(async move || {
            Html(async_graphql::http::graphiql_source("/api/graphql", None))
        }),
    );

    Router::new()
        .route("/", get(index_html))
        .route("/static/*path", get(presign_url))
        .nest("/api", api_routes)
        // For our SPA to properly function we need to respond to non-supported
        // urls with the SPA itself.
        .fallback(get(index_html))
        .layer(Extension(key))
        .layer(Extension(conn))
        // Add tracing to our service.
        .layer(TraceLayer::new_for_http())
        // Add our Graphql schema for our handler.
        .layer(Extension(schema))
        .layer(Extension(StorageApi::new()))
}