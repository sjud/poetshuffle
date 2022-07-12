use hmac::digest::KeyInit;
use hmac::Hmac;
use postmark::reqwest::PostmarkClient;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::email::{Postmark, POSTMARK_API_TRANSACTION};
use crate::graphql::schema::new_schema;
use crate::http::{JWT_SECRET, SERVER_IP, SERVER_PORT};
use crate::http::app::app;
#[cfg(feature="dev")]
use crate::http::example_data;

/// builds our HTTP server, needs DB conn for GraphQL.
pub(crate) async fn http_server(conn: DatabaseConnection) {
    // Create our key for signing JWT's.
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes())
        .expect("Expecting valid Hmac<Sha256> from slice.");
    // Normal tracing boilerplate
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

    let schema = new_schema(conn.clone(), key.clone(), email);

    // Using storage() as a base which handles arbitrary file lookups.
    // See Axum docs for standard server boilerplate.
    axum::Server::bind(&format!("{}:{}",&*SERVER_IP,&*SERVER_PORT).parse().unwrap())
        .serve(app(key, schema,conn).into_make_service())
        .await
        .unwrap();
}