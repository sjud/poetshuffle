#![feature(async_closure)]

use lazy_static::lazy_static;

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

lazy_static! {
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
}

#[tokio::main]
async fn main() {
    // Load DB url from env vars and make sure we are up on latest
    // migration.
    let connection = sea_orm::Database::connect(&*DATABASE_URL)
        .await
        .expect(&format!("Expecting DB connection given {:?}.",&*DATABASE_URL));
    Migrator::up(&connection, None)
        .await
        .expect("Expecting Successful migration.");
    // ...
    #[cfg(feature = "dev")]
    //populate_db_with_test_data(&connection).await.unwrap();
    // Spawn test_cdn server on port 8001 during development.
    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "local_cdn")]
        local_cdn::local_cdn().await
    });
    // Spawn our normal HTTP server to handle API calls.
    let server = tokio::task::spawn(async move { http_server::http_server(connection).await });
    // We run all processes until the first error.
    try_join!(test_cdn, server).unwrap();
}
