use crate::graphql::schema::PoetShuffleSchema;
use crate::types::auth::Auth;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::StatusCode;
use axum::Extension;
use axum::body::Bytes;
use axum::extract::Path;
use axum::response::{Html, Redirect};
use crate::storage::StorageApi;
use tracing::instrument;
use crate::http::handle_http_error;

#[instrument]
pub async fn index_html(Extension(storage_api):Extension<StorageApi>) -> Result<Html<Bytes>,StatusCode> {
    let data = storage_api.get_index_file()
        .await
        .map_err(|err|
            handle_http_error(err)
        )?;
    Ok(Html(data.into()))
}
#[instrument]
pub async fn presign_url(
    Path(path): Path<String>,
    Extension(storage_api):Extension<StorageApi>) -> Result<Redirect,StatusCode> {
    Ok(Redirect::temporary(
        &storage_api.presigned_url(&path).await
            .map_err(|err|
                handle_http_error(err)
            )?
    ))
}



/// Executes GraphQL requests against out schema stored in extensions.
#[instrument(skip_all)]
pub async fn graphql_handler(
    schema: Extension<PoetShuffleSchema>,
    auth: Auth,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, StatusCode> {
    Ok(schema.execute(req.into_inner().data(auth)).await.into())
}
pub async fn health_check() -> StatusCode {
    StatusCode::OK
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
    use crate::http::app;

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
