use axum::body::Body;
use axum::extract::{BodyStream, FromRequest, RequestParts};
use axum::http::{HeaderMap, StatusCode};
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;
use crate::types::auth::Auth;
use super::*;
use sea_orm::EntityTrait;

const MAX_SIZE : usize = 10_000;

pub fn upload_router() -> Router{
    Router::new()
        .route("/poem_audio",post(upload_poem_audio))

}
pub async fn upload_poem_audio(
    _:UploadPoemAuth,
    headers:HeaderMap,
    body:Bytes,
    Extension(storage_api):Extension<StorageApi>,
) -> Result<(),StatusCode>{
    let uuid = headers.get("uuid").ok_or(
        StatusCode::BAD_REQUEST
    )?.to_str()
        .map_err(|_|
        StatusCode::BAD_REQUEST
    )?;
    storage_api.store_file(format!("static/files/poem/audio/{}", uuid),
                           body.to_vec())
        .await
        .map_err(|err|{
            tracing::error!("{:?}",err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(())
}

pub struct UploadPoemAuth(Auth);
#[async_trait::async_trait]
impl FromRequest<axum::body::Body> for UploadPoemAuth{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let auth = req.extract::<Auth>().await?;
        let extensions = req.extensions();
        let headers = req.headers();
        let content_length = headers
            .get("content-length")
            .ok_or(StatusCode::LENGTH_REQUIRED)?
            .to_str()
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            .parse::<usize>()
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
        if content_length > MAX_SIZE {
            return Err(StatusCode::PAYLOAD_TOO_LARGE)
        }
        let poem_uuid = headers.get("uuid").ok_or(
            StatusCode::BAD_REQUEST
        )?.to_str()
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            .parse::<u128>()
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
        let db = extensions.get::<DatabaseConnection>().ok_or(
            StatusCode::INTERNAL_SERVER_ERROR
        )?;
        if let Some(poem) = entity::poems::Entity::find_by_id(
            sea_orm::prelude::Uuid::from_u128(poem_uuid))
            .one(db).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?{
            if auth.can_edit_poem(&poem) {
                Ok(Self(auth))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(StatusCode::NOT_FOUND)
        }

    }
}
