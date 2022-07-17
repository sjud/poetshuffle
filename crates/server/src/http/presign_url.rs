use std::str::FromStr;
use axum::extract::{FromRequest, RequestParts};
use axum::body::Body;
use axum::Json;
use entity::sea_orm_active_enums::SetStatus;
use crate::http::upload::{FileTypeHeader, TabCatHeader, UuidHeader};
use crate::types::auth::Auth;
use sea_orm::{ColumnTrait, EntityTrait,QueryFilter};
use shared::{FileType,TableCategory};
use crate::storage::storage_path_relative;

use super::*;
#[tracing::instrument]
pub async fn presign_url_as_string(
    PathForPresignedUrl(path):PathForPresignedUrl,
    Extension(storage_api):Extension<StorageApi>) -> Json<Option<String>> {
    //TODO differentiate not found and other errors.
    match storage_api.presigned_url(&path).await {
        Ok(path) => Json(Some(path)),
        Err(err) => {
            handle_http_error(err);
            Json(None)
        }
    }
}



#[derive(PartialEq,Clone,Debug)]
pub struct PathForPresignedUrl(String);

#[async_trait::async_trait]
impl FromRequest<Body> for PathForPresignedUrl {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let auth = req.extract::<Auth>().await?;
        let uuid = sea_orm::prelude::Uuid::from_u128(
            req.extract::<UuidHeader>().await?.0.as_u128()
        );
        let table_cat = req.extract::<TabCatHeader>().await?.0;
        let file_ty = req.extract::<FileTypeHeader>().await?.0;
        let db = req
            .extensions()
            .get::<DatabaseConnection>()
            .ok_or("Can't find DB extension.")
            .map_err(|err|handle_http_error(err))?;        // Find the underlying db item
        let set_uuid = match table_cat {
            TableCategory::Intros => {
                if let Some(intro) = entity::intros::Entity::find_by_id(
                   uuid
                ).one(db)
                    .await
                    .map_err(|err|handle_http_error(err))? {
                    Ok(intro.set_uuid)
                }     else {
                    Err(handle_http_error("Intro not found."))
                }
            }
            TableCategory::Poems => {
                if let Some(poem) = entity::poems::Entity::find_by_id(
                    uuid
                ).one(db)
                    .await
                    .map_err(|err|handle_http_error(err))? {
                    Ok(poem.set_uuid)
                    }
                else {
                    Err(handle_http_error("Poem not found."))
                }
            }
            TableCategory::Banters => {
                //TODO give banter poem_uuid and set_uuid fields.
                if let Some(banter) = entity::banters::Entity::find_by_id(
                    uuid
                ).one(db)
                    .await
                    .map_err(|err|handle_http_error(err))? {
                    if let Some(poem) = entity::poems::Entity::find()
                        .filter(entity::poems::Column::BanterUuid.eq(banter.banter_uuid))
                        .one(db)
                        .await
                        .map_err(|err|handle_http_error(err))? {
                        Ok(poem.set_uuid)
                    } else {
                        Err(handle_http_error("Big err:Poem related to banter not found."))
                    }
                }
                else {
                    Err(handle_http_error("Banter not found."))
                }
            }
        }?;
        if let Some(set) = entity::prelude::Sets::find_by_id(set_uuid)
            .one(db)
            .await
            .map_err(|err|handle_http_error(err))? {
            let uuid = uuid::Uuid::from_u128(uuid.as_u128());
            if set.set_status == SetStatus::Published {
                Ok(PathForPresignedUrl(
                    storage_path_relative(table_cat,file_ty,uuid)))
            } else {
                if auth.presign_urls_for_set(&set) {
                    Ok(PathForPresignedUrl(
                        storage_path_relative(table_cat,file_ty,uuid)))
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
        } else {
            //TODO differentiate debugs/info and errors.
            Err(handle_http_error("THIS IS AN ACTUAL ERROR: Set not found."))
        }
    }
}
