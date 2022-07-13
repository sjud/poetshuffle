use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::str::FromStr;
use async_graphql::futures_util::{FutureExt, pin_mut, SinkExt, StreamExt};
use axum::body::{Body, HttpBody};
use axum::extract::{BodyStream, FromRequest, Path, RequestParts, WebSocketUpgrade};
use axum::http::{HeaderMap, StatusCode};
use crate::types::auth::{Auth, AuthUri};
use super::*;
use sea_orm::EntityTrait;
use axum::extract::ContentLengthLimit;
use axum::extract::ws::{Message, WebSocket};
use axum::headers::HeaderValue;
use axum::response::IntoResponse;
use bincode::config;
use bytes::BytesMut;
use jwt::Header;
use jwt::VerifyWithKey;
use sea_orm::prelude::Uuid;
use shared::{FileType, TableCategory, UploadHeaders};
use crate::storage::{storage_path, storage_path_relative, storage_path_ws_upload_headers};

const MAX_SIZE : u64 = 10_485_760; //10 MiB
const MAX_SIZE_USIZE : usize = MAX_SIZE as usize;
/*
#[tracing::instrument(skip_all)]
pub async fn upload_ws(
    ws:WebSocketUpgrade,
    auth:AuthUri,
    Extension(db):Extension<DatabaseConnection>,
    Extension(storage_api):Extension<StorageApi>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |mut socket:WebSocket|{
        while let Some(msg) = socket.recv().await {
            match handle_upload_msg(
                &mut socket,
                &auth.0,
                &db,
                &storage_api,
                msg.map_err(|err|format!("{:?}",err))).await {
                Ok(()) => {},
                Err(err) => handle_ws_error(&mut socket,err).await
            }
        }
    });
}
pub async fn handle_upload_msg(
    socket:&mut WebSocket,
    auth:&Auth,
    db:&DatabaseConnection,
    storage_api:&StorageApi,
    msg:Result<Message,String>) -> Result<(),String> {
    let msg = msg?;
    match msg {
        Message::Binary(data) => {
            let (msg,_) = bincode::decode_from_slice::<shared::UploadWsBinary, _>(
                &data,
                config::standard()
                    .with_limit::<MAX_SIZE_USIZE>()
            ).map_err(|err|format!("{:?}",err))?;
            let _ = auth_upload( &db,
                         msg.headers,
                         &auth).await?;
            storage_api.store_file(
                    storage_path_ws_upload_headers(msg.headers),
                    msg.file
                )
                    .await
                    .map_err(|err|format!("{:?}",err))?;
            Ok(())
        },
        Message::Text(msg) => {
            tracing::error!("{}",msg);
            Ok(())
        },
        Message::Close(_) => {
            let _ = socket.close();
            Ok(())
        },
        _ => Ok(())
    }

}
pub async fn handle_ws_error(socket:&mut WebSocket,err:impl Debug) {
    tracing::error!("{:?}",err);
    match socket.send(
        Message::Text(format!("{:?}",err)))
        .await {
        Ok(()) => {},
        Err(err) => tracing::error!("{:?}",err)
    }
}
pub async fn auth_upload(
    db:&DatabaseConnection,
    UploadHeaders{table_cat,uuid,..}:UploadHeaders,
    auth:&Auth,
) -> Result<(),String> {
    let result = match table_cat{
        TableCategory::Poems =>
            entity::poems::Entity::find_by_id(
                Uuid::from_u128(uuid.as_u128()))
                .one(db)
                .await
                .map_err(|err|format!("{:?}",err))? //MAP THE DB ERROR TO STRING AND UNWRAP IT
                .map(|poem|auth.can_edit_poem_v2(&poem))  //MAP THE OPTION INTO A OPTION<RESULT<T,E>>
                .ok_or("Poem not found".to_string())?, //MAP THE OPTION<RESULT<T,E>> INTO RESULT<RESULT<T,E>,E> AND UNWRAP IT
        TableCategory::Intros => auth.can_edit_intro_v2(),
        TableCategory::Banters =>
            entity::banters::Entity::find_by_id(
                Uuid::from_u128(uuid.as_u128()))
                .one(db)
                .await
                .map_err(|err|format!("{:?}",err))?
                .map(|banter|auth.can_edit_banter_v2(&banter))
                .ok_or("Banter not found".to_string())?
    };
    result
}*/

#[tracing::instrument(skip_all)]
pub async fn delete_file(
    _:UploadAuth,
    UuidHeader(uuid):UuidHeader,
    FileTypeHeader(file_type):FileTypeHeader,
    TabCatHeader(table_cat):TabCatHeader,
    Extension(storage_api):Extension<StorageApi>,
) -> Result<(),StatusCode> {
    storage_api.delete_file(
        storage_path(table_cat,file_type,uuid)
    ).await.map_err(|err|handle_http_error(err))
}
#[tracing::instrument(skip_all)]
pub async fn upload_file(
    _:UploadAuth,
    UuidHeader(uuid):UuidHeader,
    FileTypeHeader(file_type):FileTypeHeader,
    TabCatHeader(table_cat):TabCatHeader,
    Extension(storage_api):Extension<StorageApi>,
    ContentLengthLimit(body): ContentLengthLimit<Bytes,MAX_SIZE>
) -> Result<(),StatusCode> {
    storage_api.store_file(
        storage_path(table_cat,file_type,uuid),
        body.to_vec()
    )
        .await
        .map_err(|err|handle_http_error(err))
}
pub struct FileTypeFromUri(FileType);
#[async_trait::async_trait]
impl FromRequest<Body> for FileTypeFromUri {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(
            FileType::try_from(
            req.extract::<Path<HashMap<String, String>>>()
                .await
                .map_err(|err| handle_http_error(err))?
                .get("file")
                .ok_or("Can't find :file in uri")
                .map_err(|err|handle_http_error(err))?
                .as_str()
            ).map_err(|err|handle_http_error(err))?
        ))
    }
}
pub struct FileTypeHeader(pub(crate) FileType);
#[async_trait::async_trait]
impl FromRequest<Body> for FileTypeHeader {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(
            Self(
                FileType::try_from(
                    req.headers()
                    .get(FileType::header_name())
                    .ok_or("Can't find x-uuid header.")
                    .map_err(|err|handle_http_error(err))?
                    .to_str()
                    .map_err(|err|handle_http_error(err))?
                ).map_err(|err|handle_http_error(err))?
            ))
    }
}
pub struct UuidHeader(pub(crate) uuid::Uuid);
#[async_trait::async_trait]
impl FromRequest<Body> for UuidHeader {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(
            uuid::Uuid::from_str(
            req.headers()
                .get("x-uuid")
                .ok_or("Can't find x-uuid header.")
                .map_err(|err|handle_http_error(err))?
                .to_str()
                .map_err(|err|handle_http_error(err))?
            ).map_err(|err|handle_http_error(err))?
        ))
    }
}
pub struct UuidFromUri(Uuid);
#[async_trait::async_trait]
impl FromRequest<Body> for UuidFromUri {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(
            Uuid::from_str(
                &*req.extract::<Path<HashMap<String, String>>>()
                    .await
                    .map_err(|err| handle_http_error(err))?
                    .get("uuid")
                    .ok_or("Can't find :uuid in uri")
                    .map_err(|err| handle_http_error(err))?.to_owned())
            .map_err(|err|handle_http_error(err))?
        ))
    }
}

pub struct TabCatHeader(pub(crate) TableCategory);

#[async_trait::async_trait]
impl FromRequest<Body> for TabCatHeader {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        Ok(Self(
            TableCategory::try_from(
                headers.get(TableCategory::header_name())
                .ok_or("Can't find x-category header.")
                .map_err(|err| handle_http_error(err))?
                .to_str()
                .map_err(|err| handle_http_error(err))?
            ).map_err(|err|handle_http_error(err))?
        ))
    }
}
pub struct TableCategoryFromUri(TableCategory);
#[async_trait::async_trait]
impl FromRequest<Body> for TableCategoryFromUri {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(TableCategory::try_from(
            req.extract::<Path<HashMap<String, String>>>()
                .await
                .map_err(|err| handle_http_error(err))?
                .get("cat")
                .ok_or("Can't find :cat in uri")
                .map_err(|err|handle_http_error(err))?
                .as_str())
            .map_err(|err|handle_http_error(err))?))
    }
}

pub struct UploadAuth;
#[async_trait::async_trait]
impl FromRequest<Body> for UploadAuth{
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let auth = req.extract::<Auth>().await?;
        let uuid = Uuid::from_u128(
            req.extract::<UuidHeader>().await?.0.as_u128()
        );
        let tab_cat = req.extract::<TabCatHeader>().await?;
        let db = req
            .extensions()
            .get::<DatabaseConnection>()
            .ok_or("Can't find DB extension.")
            .map_err(|err|handle_http_error(err))?;
        let result = match tab_cat.0 {
            TableCategory::Poems =>
                entity::poems::Entity::find_by_id(uuid)
                    .one(db)
                    .await
                    .map_err(|err|handle_http_error(err))? // Map error ? result
                    .map(|poem|auth.can_edit_poem_v2(&poem)) // if poem exists check auth
                    .ok_or(StatusCode::NOT_FOUND)? // if it's not okay (poem doesn't exist, propagate)
                ,
            TableCategory::Intros => auth.can_edit_intro_v2(),
            TableCategory::Banters =>
                entity::banters::Entity::find_by_id(uuid)
                    .one(db)
                    .await
                    .map_err(|err| handle_http_error(err))?
                    .map(|banter| auth.can_edit_banter_v2(&banter))
                    .ok_or(StatusCode::NOT_FOUND)?
        };
        result.map_err(|err|handle_http_error(err))?;
        Ok(Self)
    }
}



