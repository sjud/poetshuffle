

use anyhow::Result;
use once_cell::sync::Lazy;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::path::Path;
use std::io::Write;
use tracing::{Level, event, instrument, span};
use std::fmt::Debug;
use axum::routing::get;
use axum::Router;
use axum::response::{Html};
use axum::body::Bytes;
lazy_static::lazy_static!{
    pub static ref SPACES_KEY: String = {
        if let Ok(spaces_key) = std::env::var("SPACES_KEY") {
            spaces_key
        } else {
            #[cfg(feature = "dev")]
            "".to_string();
            panic!("Spaces key must be set in release");
        }
    };
    pub static ref SPACES_SECRET: String = {
        if let Ok(spaces_secret) = std::env::var("SPACES_SECRET") {
            spaces_secret
        } else {
            #[cfg(feature = "dev")]
            "".to_string();
            panic!("Spaces key must be set in release");

        }
    };
}

#[derive(Clone)]
pub struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    // location_supported: bool,
}


impl std::fmt::Debug for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Storage")
            .field("name",&self.name)
            .field("region",&self.region)
            .field("credentials",&"REDACTED")
            .field("bucket",&self.bucket)
            .finish()
    }
}

#[derive(Clone)]
pub struct StorageApi{
    storage:Storage,
    bucket:Bucket,
}
impl Debug for StorageApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageApi")
            .field("storage",&self.storage)
            .field("bucket",&"Bucket info is equivalent to Storage info.")
            .finish()
    }
}
impl StorageApi{
    pub fn new() -> Self {
        let storage = Storage::new();
        let bucket = storage.instantiate_bucket();
        Self{storage,bucket}
    }
    pub(crate) async fn get_index_file(&self) -> Result<Vec<u8>> {
        #[cfg(feature="local_cdn")]
        return self.get_file("dist/index.html").await;

        self.get_file("static/dist/index.html").await
    }
    pub(crate) async fn get_file(
        &self,
        path: &str,
    ) -> Result<Vec<u8>> {
        #[cfg(feature="local_cdn")]
        return Ok(
            reqwest::get(format!("http://127.0.0.1:8001/{}", path))
                .await?
                .bytes()
                .await?
                .to_vec()
        );

        Ok(self.bucket.get_object(path).await?.0)
    }

    pub(crate) async fn presigned_url(&self, path: &str) -> Result<String> {
        #[cfg(feature="local_cdn")]
        return Ok(format!("http://127.0.0.1:8001/{}", path));


        Ok(self.bucket.presign_get(path, 60,None)?)
    }

    pub(crate) async fn store_file(&self, path: String,data: Vec<u8>) -> Result<()> {
        #[cfg(feature="local_cdn")]
        {std::fs::write(path,data)?;return Ok(());}

        let _ = self.bucket.put_object(path,&*data).await?;
        Ok(())
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            name: "do".into(),
            region: Region::DoNyc3,
            credentials: Credentials {
                access_key: Some(*SPACES_KEY),
                secret_key: Some(*SPACES_SECRET),
                security_token: None,
                session_token: None,
            },
            bucket: "poetshuffle".to_string(),
            //location_supported: true <- not sure what this does yet
        }
    }

    fn instantiate_bucket(&self) -> Bucket {
        Bucket::new(&self.bucket, self.region.clone(), self.credentials.clone())
            .expect("Expecting to be able to instantiate a bucket given data in self.")
    }
}



/*
#[cfg(feature = "local_storage")]
#[async_trait]
impl StorageAdaptor for Storage {
    #[instrument(err)]
    async fn get_file<S: AsRef<str> + Send + 'static + Debug>(
        &self,
        path: S,
    ) -> Result<Vec<u8>, Error> {
        event!(Level::INFO,"Storage Effect");

        let file = reqwest::get(format!("http://127.0.0.1:8001/{}", path.as_ref()))
            .await?
            .bytes()
            .await?;
        Ok(file.to_vec())
    }

    async fn presigned_url<S: AsRef<str> + Send + 'static>(
        &self,
        path: S,
    ) -> Result<String, Error> {
        Ok(format!("http://127.0.0.1:8001/{}", path.as_ref()))
    }

    async fn store_file(&self, path: String, data:Vec<u8>)
        -> Result<
        (), Error> {
        // Expect okay since this only in development?
        let mut file = std::fs::File::create(Path::new(&path)).expect("valid path");
        file.write_all(&*data)?;
        Ok(())
    }
}


use axum::response::{Html, Redirect};
use axum::body::Bytes;
use axum::extract::Path;
use axum::routing::get;
use axum::Router;

pub async fn get_file_from_test_cdn(Path(path): Path<String>) -> Result<Html<Bytes>, String> {
    Ok(Html(
        reqwest::get(format!("http://127.0.0.1:8001/{}", path))
            .await
            .map_err(|err| format!("{}", err))?
            .bytes()
            .await
            .map_err(|err| format!("{}", err))?,
    ))
}

async fn test_cdn_presigned_url(Path(path): Path<String>) -> Redirect {
    Redirect::to(&format!("http://127.0.0.1:8001{}", path))
}

fn test_cdn_storage() -> Router {
    Router::new().route("/static/route", get(test_cdn_presigned_url))
}

pub fn storage() -> Router {
    test_cdn_storage()
}

mod local_cdn {
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::Router;
use tokio::io;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

#[cfg(feature = "local_cdn")]
pub async fn local_cdn() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));
    let app = Router::new()
        .fallback(
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                ))
                .service(get_service(ServeDir::new("static")).handle_error(handle_error)),
        )
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

*/

