use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/set_schema.graphql",
query_path = "app_queries/pending_set_by_user.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PendingSetByUserQuery;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/set_schema.graphql",
query_path = "app_queries/create_pending_set.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct CreatePendingSetMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/set_schema.graphql",
query_path = "app_queries/update_set.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdateSetMutation;