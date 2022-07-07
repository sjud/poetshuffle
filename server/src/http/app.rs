use axum::response::Html;
use axum::{Extension, Router};
use axum::routing::{get,post};
use hmac::Hmac;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::graphql::schema::PoetShuffleSchema;
use crate::http::handlers::{graphql_handler, health_check, index_html, presign_url};
use crate::http::upload::upload_router;
use crate::storage::StorageApi;

pub fn app(key: Hmac<Sha256>, schema: PoetShuffleSchema, conn: DatabaseConnection) -> Router {
    // TODO Figure out what CORS should be in production
    let cors_layer = CorsLayer::new();
    #[cfg(feature="dev")]
        let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let api_routes = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/health_check",get(health_check))
        .nest("/upload",upload_router());

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
        .layer(cors_layer)
}