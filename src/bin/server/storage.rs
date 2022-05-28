use axum::body::Bytes;
use axum::extract::Path;
use axum::response::{Html, Redirect};
use axum::Router;
use axum::routing::get;

pub async fn get_file_from_test_cdn(Path(path): Path<String>, ) -> Result<Html<Bytes>,String> {
    Ok(Html(reqwest::get(format!("http://127.0.0.1:8001/{}", path))
        .await
        .map_err(|err|format!("{}",err))?
        .bytes()
        .await
        .map_err(|err|format!("{}",err))?))
}

async fn test_cdn_presigned_url(Path(path): Path<String>, ) -> Redirect {
    Redirect::to(&format!("http://127.0.0.1:8001{}", path))
}

fn test_cdn_storage() -> Router {
    Router::new()
        .route("/static/*route", get(test_cdn_presigned_url))
}



pub fn storage() -> Router {
    test_cdn_storage()
}