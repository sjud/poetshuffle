use crate::graphql::schema::PoetShuffleSchema;
use crate::types::Auth;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::HeaderMap;
use axum::Extension;
use entity::permissions::Model as Permissions;
use hmac::Hmac;
use jwt::VerifyWithKey;
use reqwest::StatusCode;
use sha2::Sha256;
use std::collections::BTreeMap;

/// Executes GraphQL requests against out schema stored in extensions.
pub async fn graphql_handler(
    schema: Extension<PoetShuffleSchema>,
    Extension(key): Extension<Hmac<Sha256>>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, StatusCode> {
    let auth = match headers.get("x-authorization") {
        Some(header) => match header.to_str() {
            Ok(str) => {
                let perm: BTreeMap<String, String> = str.verify_with_key(&key).map_err(|err| {
                    tracing::error!("verify {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
                let perm: Permissions = serde_json::from_str(&*perm["sub"]).map_err(|err| {
                    tracing::error!("from {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
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
    Ok(schema.execute(req.into_inner().data(auth)).await.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_graphql_handler() {

    }
}
