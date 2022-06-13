use super::*;
use crate::{ADMIN_PASS, ADMIN_USER};
use entity::sea_orm_active_enums::UserRole;
use entity::permissions::{
    ActiveModel as ActivePermissions,
    Model as Permissions,
};
use hmac::Hmac;
use jwt::SignWithKey;
use std::collections::BTreeMap;
use crate::graphql::resolvers::login::find_login_by_email;
use crate::types::Auth;
#[derive(Default)]
pub struct AdminMutation;

#[Object]
impl AdminMutation{
    /// This admin checks our environment variable for
    /// user credentials as opposed to a DB
    /// and it hands out the SuperAdmin user role which
    /// can promote users to administrators.
    async fn admin_login (
        &self,
        ctx: &Context<'_>,
        email: String,
        pass: String,
    ) -> Result<String> {
        if email == *ADMIN_USER && pass == *ADMIN_PASS {
            let key = ctx.data::<Hmac<Sha256>>()?;
            let mut claims = BTreeMap::new();
            claims.insert("sub", Permissions{
                user_uuid:Uuid::nil(),
                user_role:UserRole::SuperAdmin,
            });
            Ok(claims
                .sign_with_key(key)?)
        } else {
            Err(Error::new("Nuh-uh-uh."))
        }
    }
    async fn promote_user(
        &self,
        ctx: &Context<'_>,
        email:String,
        new_user_role:UserRole,
    ) -> Result<String> {
        let auth = ctx.data::<Auth>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        let key = ctx.data::<Hmac<Sha256>>()?;
        if let Some(login) = find_login_by_email(db,&email)
            .await? {
            if auth.can_issue_promotion(new_user_role) {
                ActivePermissions {
                    user_uuid: Set(login.user_uuid),
                    user_role: Set(new_user_role),
                }.update(db).await?;
                Ok("Role updated.".into())
            } else {
                Err("UnAuthorized".into())
            }
        } else {
            Err("Can't find login from email.".into())
        }
    }

}