use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/login.graphql",
    response_derives = "Serialize,PartialEq"
)]
pub struct LoginMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/register.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct RegisterMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/validate_registration.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct ValidateRegistrationMutation;