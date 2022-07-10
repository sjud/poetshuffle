use std::str::FromStr;
use axum::body::Body;
use axum::extract::{FromRequest, RequestParts};
use axum::http::{HeaderMap, StatusCode};
use crate::types::auth::Auth;
use super::*;
use sea_orm::EntityTrait;
use axum::extract::ContentLengthLimit;
use axum::headers::HeaderValue;
use jwt::Header;

const MAX_SIZE : u64 = 10_485_760; //10 MiB


#[tracing::instrument(skip_all)]
pub async fn upload(
    _:UploadAuth,
    UuidHeader(uuid):UuidHeader,
    file_type:FileType,
    file_ext:FileExt,
    table_cat:TableCategory,
    body:ContentLengthLimit<Bytes,MAX_SIZE>,
    Extension(storage_api):Extension<StorageApi>,
) -> Result<(),StatusCode>{
    //TODO will this work in production?
    storage_api.store_file(
        table_cat.storage_path(file_type,uuid),
        body.to_vec())
        .await
        .map_err(|err|handle_http_error(err))?;
    Ok(())
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
pub enum FileExt{
    Mp3,
    Ogg,
    Txt
}
#[async_trait::async_trait]
impl FromRequest<Body> for FileExt{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let ext = headers
            .get("x-file-ext")
            .ok_or("Can't find x-file-ext header.")
            .map_err(|err|handle_http_error(err))?
            .to_str()
            .map_err(|err|handle_http_error(err))?;
        match ext {
            "mp3" => Ok(FileExt::Mp3),
            "ogg" => Ok(FileExt::Ogg),
            "txt" => Ok(FileExt::Txt),
            _ => Err(handle_http_error(format!("Invalid file-ext {}",ext)))
        }
    }
}

pub enum TableCategory{
    Intros,
    Poems,
    Banter,
}
impl TableCategory{
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
        /*
        let file_ext = match file_ext {
            FileExt::Txt => "txt",
            FileExt::Mp3 => "mp3",
            FileExt::Ogg => "ogg",
        };*/
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
        /*
        let file_ext = match file_ext {
            FileExt::Txt => "txt",
            FileExt::Mp3 => "mp3",
            FileExt::Ogg => "ogg",
        };*/
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

pub struct UploadAuth;
#[async_trait::async_trait]
impl FromRequest<Body> for UploadAuth{
    type Rejection = StatusCode;
    #[tracing::instrument]
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let auth = req.extract::<Auth>().await?;
        let uuid = &req.extract::<UuidHeader>().await?.0;
        let table_category = req.extract::<TableCategory>().await?;
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



