use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use crate::graphql::schema::PoetShuffleSchema;

/// Executes GraphQL requests against out schema stored in extensions.
pub async fn graphql_handler(
    schema: Extension<PoetShuffleSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
