#![feature(async_closure)]

use lazy_static::lazy_static;
use sea_orm::{prelude::Uuid,Set,ActiveModelTrait};


use tokio::try_join;

//use crate::graphql::dev::populate_db_with_test_data;
use crate::storage::storage;
use migration::{Migrator, MigratorTrait};

mod graphql;
mod handlers;
mod http_server;
mod local_cdn;
mod storage;
mod types;
mod email;
mod auth;

lazy_static! {
    /// i.e postgresql://postgres:PASSWORD@0.0.0.0:5432/postgres
    pub static ref DATABASE_URL: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("DATABASE_URL").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("DATABASE_URL").unwrap()
    };
    pub static ref JWT_SECRET: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("JWT_SECRET").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("JWT_SECRET").unwrap()
    };
    pub static ref POSTMARK_API_TRANSACTION: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("POSTMARK_API_TRANSACTION").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("POSTMARK_API_TRANSACTION").unwrap()
    };
    pub static ref OUTBOUND_EMAIL: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("OUTBOUND_EMAIL").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("OUTBOUND_EMAIL").unwrap()
    };
    /// i.e https://127.0.0.1:8000/
    pub static ref URL_BASE: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("URL_BASE").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("URL_BASE").unwrap()
    };
     pub static ref ADMIN_USER: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("ADMIN_USER").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("ADMIN_USER").unwrap()
    };
       pub static ref ADMIN_PASS: String = {
        #[cfg(feature = "dev")]
        return dotenv_codegen::dotenv!("ADMIN_PASS").to_string();
        #[cfg(not(feature = "dev"))]
        std::env::var("ADMIN_PASS").unwrap()
    };
}

#[tokio::main]
async fn main() {
    // Load DB url from env vars and make sure we are up on latest
    // migration.
    let conn = sea_orm::Database::connect(&*DATABASE_URL)
        .await
        .expect(&format!("Expecting DB connection given {:?}.",&*DATABASE_URL));
    Migrator::up(&conn, None)
        .await
        .expect("Expecting Successful migration.");
    // Make a nil user for Admin to reference
    let _ = entity::users::ActiveModel{
        user_uuid:Set(Uuid::nil()),
        ..Default::default()
    }.insert(&conn).await;
    // ...
    #[cfg(feature = "dev")]
    //populate_db_with_test_data(&connection).await.unwrap();
    // Spawn test_cdn server on port 8001 during development.
    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "local_cdn")]
        local_cdn::local_cdn().await
    });
    // Spawn our normal HTTP server to handle API calls.
    let server = tokio::task::spawn(async move {
        http_server::http_server(conn).await });
    // We run all processes until the first error.
    try_join!(test_cdn, server).unwrap();
}
