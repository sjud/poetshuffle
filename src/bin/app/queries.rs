use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/login.graphql",
    response_derives = "Serialize,PartialEq"
)]
pub struct LoginQuery;