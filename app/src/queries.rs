use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/login.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct LoginMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/register.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct RegisterMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/validate_registration.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct ValidateRegistrationMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/super_admin_login.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct SuperAdminLoginMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/modify_user_role.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct ModifyUserRoleMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/invite_user.graphql",
    response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct InviteUserMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/accept_invitation.graphql",
    response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct AcceptInvitationMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/pending_set_by_user.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PendingSetByUserQuery;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/create_pending_set.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct CreatePendingSetMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/update_set.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdateSetMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/update_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdatePoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/update_poem_idx.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct UpdatePoemIdxMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/add_poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct AddPoemMutation;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/poem_uuids_by_set_uuid.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PoemUuidsBySetUuidQuery;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "app_queries/poem.graphql",
response_derives = "Serialize,PartialEq,Clone,Debug"
)]
pub struct PoemQuery;
