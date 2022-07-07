use std::fmt::Debug;
use crate::email::Postmark;
use crate::graphql::schema::PoetShuffleSchema;
use crate::{
    graphql::schema::new_schema,
};
use axum::routing::get;
use axum::{Extension, Router, routing::post};
use axum::body::Bytes;
use hmac::digest::KeyInit;
use hmac::Hmac;
use postmark::reqwest::PostmarkClient;
use reqwest::StatusCode;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use handlers::health_check;
use tower_http::cors::{CorsLayer};
use handlers::graphql_handler;
use crate::email::POSTMARK_API_TRANSACTION;
use crate::http::handlers::{index_html, presign_url};
use crate::http::upload::upload_router;
use crate::storage::StorageApi;

mod handlers;
mod upload;
pub(crate) mod http_server;
mod app;

lazy_static::lazy_static!{
        pub static ref SERVER_PORT: String = {
        if let Ok(port) = std::env::var("SERVER_PORT") {
            return port;
        } else {
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("SERVER_PORT").to_string();
        }
        panic!("Requires server port, not set in .env or environment");
    };
    pub static ref SERVER_IP: String = {
        if let Ok(ip) = std::env::var("SERVER_IP") {
            return ip;
        } else {
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("SERVER_IP").to_string();
        }
        panic!("Requires SERVER_IP, not set in .env or environment");
    };
    pub static ref JWT_SECRET: String = {
        if let Ok(secret) = std::env::var("JWT_SECRET") {
            return secret;
        } else {
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("JWT_SECRET").to_string();
        }
        panic!("Requires JWT_SECRET, not set in .env or environment");
    };
}

pub fn handle_http_error(err:impl Debug) -> StatusCode {
    tracing::error!("{:?}",err);
    StatusCode::INTERNAL_SERVER_ERROR
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
