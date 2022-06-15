use super::*;
use entity::{
    invitations::Model as Invitation,
    prelude::Invitations,
};
use crate::graphql::resolvers::login::create_login_with_password_and_role;

#[derive(Default)]
pub struct PublishMutation;

#[Object]
impl PublishMutation{
    async fn accept_invitation (
        &self,
        ctx: &Context<'_>,
        password: String,
        invite_uuid: Uuid,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let invitation: Invitation = Invitations::find_by_id(invite_uuid)
            .one(db)
            .await?
            .ok_or(Error::new("Invitation not found."))?;
        create_login_with_password_and_role(
            db,
            invitation.invitee_email,
            password,
            invitation.user_role,
            invitation.invitor_uuid
        ).await?;
        entity::invitations::ActiveModel{
            invite_uuid:Set(invite_uuid),
            fufilled:Set(true),
            fufilled_ts:Set(Some(time_now())),
            ..Default::default()
        }.update(db).await?;
        Ok(format!("Account created with role {:?}, please log in\n\
        using the email that received the invitation and your password.",invitation.user_role))

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;
    use entity::sea_orm_active_enums::UserRole;
    use crate::graphql::resolvers::login::{create_login_with_password, find_login_by_email};
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::{assert_no_graphql_errors_or_print_them, key_conn_email};

    #[tokio::test]
    #[traced_test]
    async fn test_accept_invitation() {
        let (key,conn,email) = key_conn_email().await;
        let schema = new_schema(conn.clone(),key,email);
        let invitee_email = "test_invitee_email@test.com";
        let user_uuid = create_login_with_password(
            &conn,
            "test_accept_invitation@test.com".into(),
            "1234".into()).await.unwrap();
        let invite_uuid = Uuid::new_v4();
        entity::invitations::ActiveModel{
            invite_uuid: Set(invite_uuid),
            invitor_uuid: Set(user_uuid),
            invitee_email: Set(invitee_email.into()),
            user_role: Set(UserRole::Poet),
            ..Default::default()
        }.insert(&conn).await.unwrap();
        let result = schema
            .execute(async_graphql::Request::from(format!(
                "mutation {{
                acceptInvitation(password: \"1234\", inviteUuid: \"{}\")
                }}",invite_uuid
            )))
            .await;
        assert_no_graphql_errors_or_print_them(result.errors);
        // Panic if email isn't associated with a login.
        let _ = find_login_by_email(&conn, invitee_email)
            .await
            .unwrap().unwrap();
    }
}