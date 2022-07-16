use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/delete_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct DeletePoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/set_approve_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct SetApprovePoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/update_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdatePoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/update_poem_idx.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdatePoemIdxMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/add_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct AddPoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/poem_uuids_by_set_uuid.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PoemUuidsBySetUuidQuery;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/poem_schema.graphql",
query_path = "app_queries/poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PoemQuery;