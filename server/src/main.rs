#![feature(async_closure)]

use lazy_static::lazy_static;
use sea_orm::{ActiveModelTrait, prelude::Uuid, Set};

use tokio::try_join;

//use crate::graphql::dev::populate_db_with_test_data;
use migration::{Migrator, MigratorTrait};

mod auth;
mod email;
mod graphql;
mod storage;
mod types;
mod http;
mod local_cdn;

lazy_static! {
    /// i.e postgresql://postgres:PASSWORD@0.0.0.0:5432/postgres
    pub static ref DATABASE_URL: String = {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            tracing::debug!("Found DATABASE_URL in environment.\n{:?}",url);
            url
        } else {
            tracing::debug!("Attempting to use .env to get DATABASE_URL");
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("DATABASE_URL").to_string();
            panic!("Requires DATABASE URL, not set in .env or environment");
        }
    };
}

#[tokio::main]
async fn main() {
    // Load DB url from env vars and make sure we are up on latest
    // migration.
    let conn = sea_orm::Database::connect(&*DATABASE_URL)
        .await
        .expect(&format!(
            "Expecting DB connection given {:?}.",
            &*DATABASE_URL
        ));
    Migrator::up(&conn, None)
        .await
        .expect("Expecting Successful migration.");
    // Make a nil user for Admin to reference
    let _ = entity::users::ActiveModel {
        user_uuid: Set(Uuid::nil()),
        ..Default::default()
    }
    .insert(&conn)
    .await;

    let test_cdn = tokio::task::spawn(async move {
        #[cfg(feature = "local_cdn")]
        local_cdn::local_cdn().await
    });
    // Spawn our normal HTTP server to handle API calls.
    let server = tokio::task::spawn(async move {
        http::http_server(conn).await
    });
    // We run all processes until the first error.
    try_join!(test_cdn, server).unwrap();
}

