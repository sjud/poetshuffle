use std::str::FromStr;
use axum::body::Body;
use axum::extract::{FromRequest, RequestParts};
use axum::http::{HeaderMap, StatusCode};
use crate::types::auth::Auth;
use super::*;
use sea_orm::EntityTrait;
use axum::extract::ContentLengthLimit;
const MAX_SIZE : u64 = 10_485_760; //10 MiB

pub fn upload_router() -> Router{
    Router::new()
        .route("/poem_audio",post(upload_poem_audio))
}
#[tracing::instrument(skip_all)]
pub async fn upload_poem_audio(
    _:UploadPoemAuth,
    headers:HeaderMap,
    body:ContentLengthLimit<Bytes,MAX_SIZE>,
    Extension(storage_api):Extension<StorageApi>,
) -> Result<(),StatusCode>{
    let uuid = headers
        .get("x-uuid")
        .ok_or("Can't find x-uuid header.")
        .map_err(|err|handle_http_error(err))?
        .to_str()
        .map_err(|err|handle_http_error(err))?;
    //TODO will this work in production?
    storage_api.store_file(format!("static/files/poem/audio/{}", uuid),
                           body.to_vec())
        .await
        .map_err(|err|handle_http_error(err))?;
    Ok(())
}

pub struct UploadPoemAuth;
#[async_trait::async_trait]
impl FromRequest<Body> for UploadPoemAuth{
    type Rejection = StatusCode;
    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let auth = req.extract::<Auth>().await?;
        let headers = req.headers();
        let poem_uuid = headers.get("x-uuid")
            .ok_or("Can't find x-uuid header.")
            .map_err(|err|handle_http_error(err))?
            .to_str()
            .map_err(|err|handle_http_error(err))?;
        let db = req
            .extensions()
            .get::<DatabaseConnection>()
            .ok_or("Can't find DB extension.")
            .map_err(|err|handle_http_error(err))?;
        if let Some(poem) = entity::poems::Entity::find_by_id(
            sea_orm::prelude::Uuid::from_str(poem_uuid)
                .map_err(|err|handle_http_error(err))?
        ).one(db)
            .await
            .map_err(|err|handle_http_error(err))?{
            if auth.can_edit_poem(&poem) {
                Ok(Self)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(handle_http_error("Poem not found."))
        }
    }
}


