use crate::graphql::schema::PoetShuffleSchema;
use crate::types::auth::Auth;
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
            Ok(token_str) => {
                let claims: BTreeMap<String, Permissions> =
                    token_str.verify_with_key(&key).map_err(|err| {
                        tracing::error!("verify {:?}", err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                let perm: Permissions =
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
    Ok(schema.execute(req.into_inner().data(auth)).await.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::auth::jwt;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::key_conn_email;
    use crate::http_server::{app, http_server};
    use axum::body::Body;
    use axum::http::Request;
    use entity::sea_orm_active_enums::UserRole;
    use sea_orm::prelude::Uuid;
    use tower::ServiceExt;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_graphql_handler_authorization() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key.clone(), email);
        let jwt = jwt(
            &key,
            entity::permissions::Model {
                user_uuid: Uuid::nil(),
                user_role: UserRole::Admin,
            },
        )
        .unwrap();
        let response = app(key.clone(),schema)
            .oneshot(
                Request::builder()
                .method("POST")
                .uri("/api/graphql")
                .header("x-authorization",jwt)
                .body(Body::from(r#"{"variables":{"email":"test@test.com","new_user_role":"ADMIN"},"query":"mutation ModifyUserRoleMutation($email: String!, $new_user_role: UserRole!) {\n  modifyUserRole(email: $email, newUserRole: $new_user_role)\n}\n","operationName":"ModifyUserRoleMutation"}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
