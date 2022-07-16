use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/banter_schema.graphql",
query_path = "app_queries/add_banter.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct AddBanterMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/banter_schema.graphql",
query_path = "app_queries/delete_banter.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct DeleteBanterMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "app_schemas/banter_schema.graphql",
query_path = "app_queries/set_approve_banter.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct SetApproveBanterMutation;