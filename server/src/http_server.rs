use crate::email::Postmark;
use crate::graphql::schema::PoetShuffleSchema;
use crate::{
    graphql::schema::new_schema, handlers::graphql_handler, storage, POSTMARK_API_TRANSACTION,
};
use axum::routing::MethodRouter;
use axum::{extract::Path, response::Html, routing::post, Extension, Router};
use hmac::digest::KeyInit;
use hmac::Hmac;
use postmark::reqwest::PostmarkClient;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    // Build our email client for our schema to send emails.
    let email = Postmark {
        client: PostmarkClient::builder()
            .base_url("https://api.postmarkapp.com/")
            .token(&*POSTMARK_API_TRANSACTION)
            .build(),
    };
    #[cfg(feature = "dev")]
    example_data(&conn).await;

    let schema = new_schema(conn, key.clone(), email);

    // Using storage() as a base which handles arbitrary file lookups.
    // See Axum docs for standard server boilerplate.
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app(key, schema).into_make_service())
        .await
        .unwrap();
}

pub fn app(key: Hmac<Sha256>, schema: PoetShuffleSchema) -> Router {
    let api_routes = Router::new().route("/graphql", post(graphql_handler));
    // For use during development.
    #[cfg(feature = "graphiql")]
    let api_routes = api_routes.route(
        "/graphiql",
        axum::routing::any(async move || {
            Html(async_graphql::http::graphiql_source("/api/graphql", None))
        }),
    );

    storage()
        .route("/", get_index_html())
        .nest("/api", api_routes)
        // For our SPA to properly function we need to respond to non-supported
        // urls with the SPA itself.
        .fallback(get_index_html())
        .layer(Extension(key))
        // Add tracing to our service.
        .layer(TraceLayer::new_for_http())
        // Add our Graphql schema for our handler.
        .layer(Extension(schema))
}

pub fn get_index_html() -> MethodRouter {
    axum::routing::get(async move || {
        storage::get_file_from_test_cdn(Path("dist/index.html".to_string())).await
    })
}

#[cfg(feature = "dev")]
pub async fn example_data(conn: &DatabaseConnection) {
    use crate::graphql::resolvers::login::create_login_with_password;
    let _ = create_login_with_password(conn, "dev@test.com".into(), "1234".into()).await;
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_util::key_conn_email;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_index_app() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key.clone(), email);
        let response = app(key.clone(), schema)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
