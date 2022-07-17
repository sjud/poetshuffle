use anyhow::Result;
use entity::permissions::Model as Permission;
use entity::sea_orm_active_enums::{SetStatus, UserRole};
use sea_orm::prelude::Uuid;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use axum::body::{Body};
use axum::extract::{FromRequest, Path, RequestParts};
use axum::http::StatusCode;
use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;
use crate::http::handle_http_error;

pub struct Auth(pub Option<Permission>);
pub struct AuthUri(pub Auth);
#[async_trait::async_trait]
impl FromRequest<Body> for AuthUri {
    type Rejection = StatusCode;
    async fn from_request(req: &mut RequestParts<Body>)
        -> Result<Self, Self::Rejection> {
        let paths = req.extract::<Path<HashMap<String, String>>>()
            .await
            .map_err(|err| handle_http_error(err))?;
        let jwt = paths.0.get("jwt")
            .ok_or("Can't find jwt")
            .map_err(|err| handle_http_error(err))?;
        let key = req.extensions().get::<Hmac<Sha256>>()
            .ok_or("Can't find key in extensions.")
            .map_err(|err| handle_http_error(err))?;
        let claims: BTreeMap<String, entity::permissions::Model> =
            jwt.verify_with_key(key).map_err(|err| {
                    tracing::error!("verify {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
        let perm =
                claims.get("sub").ok_or(StatusCode::BAD_REQUEST)?.to_owned();
        Ok(AuthUri(Auth(Some(perm))))
    }
}
#[async_trait::async_trait]
impl FromRequest<Body> for Auth {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>)
        -> std::result::Result<Self, Self::Rejection> {
        let headers = req.headers();
        let key = req.extensions().get::<Hmac<Sha256>>()
            .ok_or("Can't find key in extensions.")
            .map_err(|err|handle_http_error(err))?;
        let auth = match headers.get("x-authorization") {
            Some(header) => match header.to_str() {
                Ok(token_str) => {
                    let claims: BTreeMap<String, Permission> =
                        token_str.verify_with_key(key).map_err(|err| {
                            tracing::error!("verify {:?}", err);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?;
                    let perm: Permission =
                        claims.get("sub").ok_or(StatusCode::BAD_REQUEST)?.to_owned();
                    Auth(Some(perm))
                }
                Err(err) => {
                    tracing::error!(" to {:?}", err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)?;
                    Auth(None)
                }
            },
            None => Auth(None),
        };
        Ok(auth)
    }
}

impl Auth {
    pub fn can_upload_poem(&self, poem:&entity::poems::Model) -> Result<(),String> {
        if !poem.approved {
            if let Some(permission) = &self.0 {
                // If you created the poem you can edit the poem.
                if poem.originator_uuid == permission.user_uuid {
                    Ok(())
                } else {
                    Err("Unauthorized".into())
                }
            } else {
                Err("Authorization not found.".into())
            }
        } else {
            Err("Poem has already been approved and can't be edited.".into())
        }
    }
    pub fn can_upload_banter(&self, item:&entity::banters::Model) -> Result<(),String> {
        if !item.approved {
            if let Some(permission) = &self.0 {
                // If you created the poem you can edit the poem.
                if item.originator_uuid == permission.user_uuid {
                    Ok(())
                } else {
                    Err("Unauthorized".into())
                }
            } else {
                Err("Authorization not found.".into())
            }
        } else {
            Err("Banter has already been approved and can't be edited.".into())
        }
    }
    pub fn can_upload_intro(&self) -> Result<(),String> {
        if let Some(permission) = &self.0 {
            if OrdRoles(permission.user_role) >= OrdRoles(UserRole::Moderator) {
                Ok(())
            } else {
                Err(String::from("Unauthorized."))
            }
        } else {
            Err(String::from("Permission not found."))
        }
    }
    pub fn presign_urls_for_set(&self, set: &entity::sets::Model) -> bool {
        if let Some(permission) = &self.0 {
            if OrdRoles(permission.user_role) >= OrdRoles(UserRole::Moderator) {
                true
            } else {
                set.originator_uuid == permission.user_uuid
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OrdRoles(pub UserRole);
impl PartialOrd for OrdRoles {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = match self.0 {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        let other_val = match other.0 {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        if self_val < other_val {
            Some(Ordering::Less)
        } else if self_val > other_val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
