use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::str::FromStr;
use async_graphql::futures_util::{FutureExt, pin_mut, SinkExt};
use axum::body::Body;
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
use jwt::Header;
use jwt::VerifyWithKey;
use shared::UploadHeaders;
use crate::storage::{storage_path_relative, storage_path_ws_upload_headers};

const MAX_SIZE : u64 = 10_485_760; //10 MiB
const MAX_SIZE_USIZE : usize = MAX_SIZE as usize;
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
            socket.close();
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
        shared::TableCategory::Poems =>
            entity::poems::Entity::find_by_id(
                sea_orm::prelude::Uuid::from_u128(uuid.as_u128()))
                .one(db)
                .await
                .map_err(|err|format!("{:?}",err))? //MAP THE DB ERROR TO STRING AND UNWRAP IT
                .map(|poem|auth.can_edit_poem_v2(&poem))  //MAP THE OPTION INTO A OPTION<RESULT<T,E>>
                .ok_or("Poem not found".to_string())?, //MAP THE OPTION<RESULT<T,E>> INTO RESULT<RESULT<T,E>,E> AND UNWRAP IT
        shared::TableCategory::Intros => auth.can_edit_intro_v2(),
        shared::TableCategory::Banter =>
            entity::banters::Entity::find_by_id(
                sea_orm::prelude::Uuid::from_u128(uuid.as_u128()))
                .one(db)
                .await
                .map_err(|err|format!("{:?}",err))?
                .map(|banter|auth.can_edit_banter_v2(&banter))
                .ok_or("Banter not found".to_string())?
    };
    result
}

#[tracing::instrument(skip_all)]
pub async fn upload(
    _:UploadAuth,
    UuidFromUri(uuid):UuidFromUri,
    FileTypeFromUri(file_type):FileTypeFromUri,
    TableCategoryFromUri(table_cat):TableCategoryFromUri,
    ws: WebSocketUpgrade,
    Extension(storage_api):Extension<StorageApi>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |mut socket:WebSocket| {
        if let Some(Ok(Message::Binary(data))) = socket.recv().await {
            let _ = storage_api.store_file(
                table_cat.storage_path(file_type,uuid),
                data
            )
                .await
                .map_err(|err|socket
                    .send(
                        Message::Text(
                            format!("{:?}",err)
                    )));
        } else {
            tracing::error!("Did not receive some okay binary data.");
        }
    })
}
pub struct FileTypeFromUri(FileType);
#[async_trait::async_trait]
impl FromRequest<Body> for FileTypeFromUri {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        match req.extract::<Path<HashMap<String, String>>>()
            .await
            .map_err(|err| handle_http_error(err))?
            .get("file")
            .ok_or("Can't find :file in uri")
            .map_err(|err|handle_http_error(err))?
            .as_str() {
            "transcript" => Ok(Self(FileType::Transcript)),
            "audio" => Ok(Self(FileType::Audio)),
            file_type => Err(handle_http_error(
                format!("Value of :file path was {}\
                 instead of 'audio' or 'transcript'.",file_type)))
        }
    }
}
pub enum FileType{
    Audio,
    Transcript
}
#[async_trait::async_trait]
impl FromRequest<Body> for FileType {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let file_type = headers
            .get("x-file-type")
            .ok_or("Can't find x-uuid header.")
            .map_err(|err|handle_http_error(err))?
            .to_str()
            .map_err(|err|handle_http_error(err))?;
        if file_type == "transcript"{
            Ok(Self::Transcript)
        } else if file_type == "audio" {
            Ok(Self::Audio)
        } else {
            Err(handle_http_error(
                format!("Value of x-file-type header was {}\
                 instead of 'audio' or 'transcript'.",file_type)))
        }
    }
}
pub struct UuidHeader(pub(crate) String);
#[async_trait::async_trait]
impl FromRequest<Body> for UuidHeader {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let uuid = headers
            .get("x-uuid")
            .ok_or("Can't find x-uuid header.")
            .map_err(|err|handle_http_error(err))?
            .to_str()
            .map_err(|err|handle_http_error(err))?;
        Ok(Self(uuid.to_string()))
    }
}
pub struct UuidFromUri(String);
#[async_trait::async_trait]
impl FromRequest<Body> for UuidFromUri {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(req.extract::<Path<HashMap<String, String>>>()
            .await
            .map_err(|err| handle_http_error(err))?
            .get("uuid")
            .ok_or("Can't find :uuid in uri")
            .map_err(|err|handle_http_error(err))?.to_owned()))
    }
}
pub enum TableCategory{
    Intros,
    Poems,
    Banter,
}
impl TableCategory{
    pub fn from_str(val:&str) -> Result<Self,StatusCode> {
        match val {
            "poem" => {
                Ok(TableCategory::Poems)
            },
            "banter" => {
                Ok(TableCategory::Banter)
            },
            "intro" => {
                Ok(TableCategory::Intros)
            }
            other => {
                Err(handle_http_error(
                    format!("x-category was {} \
                            instead of poem, banter, intro",other)
                ))
            }
        }
    }
    pub fn storage_path(&self,file_type:FileType,uuid:String) -> String{
        let table_cat = match self {
            TableCategory::Intros => "intro",
            TableCategory::Poems => "poem",
            TableCategory::Banter => "banter",
        };
        let file_ty = match file_type {
            FileType::Audio => "audio",
            FileType::Transcript => "transcript"
        };
        format!("static/files/{}/{}/{}",table_cat,file_ty,uuid)
    }
    pub fn storage_path_relative(&self,file_type:FileType,uuid:String) -> String {
        let table_cat = match self {
            TableCategory::Intros => "intro",
            TableCategory::Poems => "poem",
            TableCategory::Banter => "banter",
        };
        let file_ty = match file_type {
            FileType::Audio => "audio",
            FileType::Transcript => "transcript"
        };

        format!("files/{}/{}/{}",table_cat,file_ty,uuid)
    }
}


#[async_trait::async_trait]
impl FromRequest<Body> for TableCategory {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        match headers.get("x-category")
            .ok_or("Can't find x-category header.")
            .map_err(|err|handle_http_error(err))?
            .to_str()
            .map_err(|err|handle_http_error(err))? {
            "poem" => {
                    Ok(TableCategory::Poems)
            },
            "banter" => {
                    Ok(TableCategory::Banter)
            },
            "intro" => {
                    Ok(TableCategory::Intros)
            }
            other => {
                Err(handle_http_error(
                        format!("x-category was {} \
                            instead of poem, banter, intro",other)
                ))
            }
        }
    }
}
pub struct TableCategoryFromUri(TableCategory);
#[async_trait::async_trait]
impl FromRequest<Body> for TableCategoryFromUri {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Self(TableCategory::from_str(
            req.extract::<Path<HashMap<String, String>>>()
                .await
                .map_err(|err| handle_http_error(err))?
                .get("cat")
                .ok_or("Can't find :cat in uri")
                .map_err(|err|handle_http_error(err))?)?))
    }
}

pub struct UploadAuth;
#[async_trait::async_trait]
impl FromRequest<Body> for UploadAuth{
    type Rejection = StatusCode;
    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let paths = req.extract::<Path<HashMap<String, String>>>()
            .await.map_err(|err| handle_http_error(err))?;
        let jwt = paths.0.get("jwt")
            .ok_or("Can't find jwt")
            .map_err(|err| handle_http_error(err))?;
        let key = req.extensions().get::<Hmac<Sha256>>()
            .ok_or("Can't find key in extensions.")
            .map_err(|err| handle_http_error(err))?;
        let auth = {
            let claims: BTreeMap<String, entity::permissions::Model> =
                jwt.verify_with_key(key).map_err(|err| {
                    tracing::error!("verify {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
             let perm =
            claims.get("sub").ok_or(StatusCode::BAD_REQUEST)?.to_owned();
            Auth(Some(perm))
        };
        let uuid = paths.0.get("uuid")
            .ok_or("Can't find jwt")
            .map_err(|err| handle_http_error(err))?;
        let table_category = TableCategory::from_str(
            paths.0.get("cat")
            .ok_or("Can't find jwt")
            .map_err(|err| handle_http_error(err))?)?;
        let db = req
            .extensions()
            .get::<DatabaseConnection>()
            .ok_or("Can't find DB extension.")
            .map_err(|err|handle_http_error(err))?;
        match table_category {
            TableCategory::Poems => {
                if let Some(poem) = entity::poems::Entity::find_by_id(
                    sea_orm::prelude::Uuid::from_str(uuid)
                        .map_err(|err|handle_http_error(err))?
                ).one(db)
                    .await
                    .map_err(|err|handle_http_error(err))?{
                    if auth.can_edit_poem(&poem) {
                        Ok(Self)
                    } else {
                        Err(StatusCode::UNAUTHORIZED)
                    }
                }     else {
                Err(handle_http_error("Poem not found."))
                }
            },
            TableCategory::Intros => {
                    if auth.can_edit_intro() {
                        Ok(Self)
                    } else {
                        Err(StatusCode::UNAUTHORIZED)
                    }
                },
            TableCategory::Banter => {
                if let Some(banter) = entity::banters::Entity::find_by_id(
                    sea_orm::prelude::Uuid::from_str(uuid)
                        .map_err(|err|handle_http_error(err))?
                ).one(db)
                    .await
                    .map_err(|err|handle_http_error(err))?{
                    if auth.can_edit_banter(&banter) {
                        Ok(Self)
                    } else {
                        Err(StatusCode::UNAUTHORIZED)
                    }
                } else {
                    Err(handle_http_error("Banter not found."))
                }
            }
        }
    }
}



