use std::collections::BTreeMap;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use axum::http::{HeaderMap};
use hmac::Hmac;
use jwt::VerifyWithKey;
use reqwest::StatusCode;
use sha2::Sha256;
use crate::graphql::schema::PoetShuffleSchema;
use crate::types::Auth;
use entity::permissions::Model as Permissions;

/// Executes GraphQL requests against out schema stored in extensions.
pub async fn graphql_handler(
    schema: Extension<PoetShuffleSchema>,
    Extension(key): Extension<Hmac<Sha256>>,
    headers:HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse,StatusCode> {
    let auth = match headers.get("authorization") {
        Some(header) => match header.to_str() {
            Ok(str) => {
                let perm :BTreeMap<String,String>= str.verify_with_key(&key).map_err(|err|{
                    tracing::error!("{:?}",err);
                    StatusCode::INTERNAL_SERVER_ERROR})?;
                let perm:Permissions= serde_json::from_str(&*perm["sub"])
                    .map_err(|err|{
                        tracing::error!("{:?}",err);
                        StatusCode::INTERNAL_SERVER_ERROR})?;
                Auth(Some(perm))
            },
            Err(err) => {
                tracing::error!("{:?}",err);
                Auth(None)}
        },
        None => Auth(None)
    };
    Ok(schema.execute(req
        .into_inner()
        .data(auth)
    ).await.into())
}
