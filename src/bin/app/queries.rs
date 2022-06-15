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
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/super_admin_login.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct SuperAdminLoginMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/modify_user_role.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct ModifyUserRoleMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/invite_user.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct InviteUserMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/accept_invitation.graphql",
response_derives = "Serialize,PartialEq"
)]
pub struct AcceptInvitationMutation;