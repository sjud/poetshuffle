use crate::{graphql::schema::new_schema, handlers::graphql_handler, POSTMARK_API_TRANSACTION, storage};
use axum::{extract::Path, response::Html, routing::post, Extension, Router};
use hmac::digest::KeyInit;
use hmac::Hmac;
use postmark::reqwest::PostmarkClient;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::email::Postmark;

/// builds our HTTP server, needs DB conn for GraphQL.
pub(crate) async fn http_server(conn: DatabaseConnection) {
    // Create our key for signing JWT's.
    let key: Hmac<Sha256> = Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
        .expect("Expecting valid Hmac<Sha256> from slice.");
    // Normal tracing boilerplate to get traces, see tracing docs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let api_routes = Router::new().route("/graphql", post(graphql_handler));
    // For use during development.
    #[cfg(feature = "graphiql")]
    let api_routes = api_routes.route(
        "/graphiql",
        axum::routing::any(async move || {
            Html(async_graphql::http::graphiql_source("/api/graphql", None))
        }),
    );
    // Build our email client for our schema to send emails.
    let client = Postmark{
        client:
        PostmarkClient::builder()
            .base_url("https://api.postmarkapp.com/")
            .token(&*POSTMARK_API_TRANSACTION)
            .build()
    };

    // Using storage() as a base which handles arbitrary file lookups.
    let app = storage()
        .route(
            "/",
            axum::routing::get(async move || {
                storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
            }),
        )
        .nest("/api", api_routes)
        // For our SPA to properly function we need to respond to non-supported
        // urls with the SPA itself.
        .fallback(axum::routing::get(async move || {
            storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
        }))
        // Add tracing to our service.
        .layer(TraceLayer::new_for_http())
        .layer(Extension(key.clone()))
        // Add our Graphql schema for our handler.
        .layer(Extension(new_schema(conn,key,client)));
    // See Axum docs for standard server boilerplate.
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
