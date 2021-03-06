use super::*;
use crate::email::{Email, Postmark};
use crate::graphql::resolvers::login::find_login_by_email;
use crate::types::auth::{Auth, OrdRoles};
use entity::permissions::{ActiveModel as ActivePermissions, Model as Permissions};
use entity::sea_orm_active_enums::UserRole;
use hmac::Hmac;
use jwt::SignWithKey;
use std::collections::BTreeMap;
use crate::graphql::guards::*;
use entity::prelude::Logins;
lazy_static::lazy_static!{
    pub static ref ADMIN_USER: String = {
        if let Ok(user_name) = std::env::var("ADMIN_USER") {
            user_name
        } else {
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("ADMIN_USER").to_string();
            panic!("Requires ADMIN_USER, not set in .env or environment");
        }
    };
    pub static ref ADMIN_PASS: String = {
        if let Ok(password) = std::env::var("ADMIN_PASS") {
            password
        } else {
            #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("ADMIN_PASS").to_string();
            panic!("Requires ADMIN_PASS, not set in .env or environment");
        }
    };
}
#[derive(Default)]
pub struct AdminMutation;
#[cfg(test)]
use crate::email::TestEmail;

#[Object]
impl AdminMutation {
    /// This admin checks our environment variable for
    /// user credentials as opposed to a DB
    /// and it hands out the SuperAdmin user role which
    /// can promote users to administrators.
    async fn super_admin_login(
        &self,
        ctx: &Context<'_>,
        email: String,
        pass: String,
    ) -> Result<String> {
        if email == *ADMIN_USER && pass == *ADMIN_PASS {
            let key = ctx.data::<Hmac<Sha256>>()?;
            let mut claims = BTreeMap::new();
            claims.insert(
                "sub",
                Permissions {
                    user_uuid: Uuid::nil(),
                    user_role: UserRole::SuperAdmin,
                },
            );
            Ok(claims.sign_with_key(key)?)
        } else {
            Err(Error::new("Admin credentials invalid."))
        }
    }
    #[graphql(guard = "MinRoleGuard::new(new_user_role)")]
    async fn modify_user_role(
        &self,
        ctx: &Context<'_>,
        email: String,
        new_user_role: UserRole,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let login =  Logins::find()
            .having(entity::logins::Column::Email.eq(email))
            .group_by(entity::logins::Column::UserUuid)
            .one(db)
            .await?
            .ok_or(Error::new("Can't find user to promote given email"))?;
            ActivePermissions {
                user_uuid: Set(login.user_uuid),
                user_role: Set(new_user_role),
            }.update(db).await?;
        Ok("Role updated.".into())

    }
    #[graphql(guard = "MinRoleGuard::new(user_role)")]
    async fn invite_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        user_role: UserRole,
    ) -> Result<String> {
        let invitor_uuid = ctx.data::<Auth>()?
            .0
            .as_ref()
            .ok_or("Can't find permission")?
            .user_uuid;
        let db = ctx.data::<DatabaseConnection>()?;
        #[cfg(test)]
        let email_client = ctx.data::<TestEmail>()?;
        #[cfg(not(test))]
        let email_client = ctx.data::<Postmark>()?;
        let login = Logins::find()
            .having(entity::logins::Column::Email.eq(email.clone()))
            .group_by(entity::logins::Column::UserUuid)
            .one(db)
            .await?
            .ok_or(Error::new("Can't find user to promote given email"))?;
        let perm: entity::permissions::Model =
            entity::prelude::Permissions::find_by_id(login.user_uuid)
                .one(db)
                .await?
                .ok_or("Server error: Login without valid permissions.")?;
        if OrdRoles(perm.user_role) >= OrdRoles(user_role) {
            Err("User equivalent or greater role")?;
        }
        let invite_uuid = Uuid::new_v4();
        entity::invitations::ActiveModel {
            invite_uuid: Set(invite_uuid),
            invitor_uuid: Set(invitor_uuid),
            invitee_email: Set(email.clone()),
            user_role: Set(user_role),
            ..Default::default()
        }
            .insert(db)
            .await?;
        email_client.invite_user(email, invite_uuid.into()).await?;
        Ok("User invited".into())
    }
}
/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::email::{MockEmail, TestEmail};
    use crate::graphql::resolvers::login::create_login_with_password;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::key_conn_email;
    use crate::DATABASE_URL;
    use hmac::digest::KeyInit;
    use std::sync::{Arc, Mutex};
    use tracing_test::traced_test;
    #[tokio::test]
    async fn test_super_admin_login() {
        let (key, conn, email) = key_conn_email().await;
        let user = &*ADMIN_USER;
        let pass = &*ADMIN_PASS;

        let schema = new_schema(conn, key, email);

        let result = schema
            .execute(&format!(
                "mutation {{
                superAdminLogin(email: \"{}\", pass: \"{}\")
                }}",
                user, pass
            ))
            .await;
        eprintln!("{:?}", result.errors);
        assert!(result.errors.is_empty());
        //assert_eq!(result.data.to_string(),
        //           "{login: \"SOME JWT STRING\"}".to_string()); <- how to test valid jwt?
        // Our return value is some token, let's test is later in an integration test.
        // bad pass
        let result = schema
            .execute(&format!(
                "mutation {{
                superAdminLogin(email: \"{}\", pass: \"bad_pass\")
                }}",
                user
            ))
            .await;
        assert!(!result.errors.is_empty()); // Should error.
                                            // bad user
        let result = schema
            .execute(&format!(
                "mutation {{
                superAdminLogin(email: \"bad_user\", pass: \"{}\")
                }}",
                pass
            ))
            .await;
        assert!(!result.errors.is_empty()); // Should error.
    }
    #[tokio::test]
    async fn promote_user() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key, email);
        let user_uuid =
            create_login_with_password(&conn, "soon_to_be_poet@test.com".into(), "1234".into())
                .await
                .unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation {
                modifyUserRole(email: \"soon_to_be_poet@test.com\", newUserRole: \"POET\")
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: Uuid::nil(),
                    user_role: UserRole::Moderator,
                }))),
            )
            .await;
        eprintln!("{:?}", result.errors);
        assert!(result.errors.is_empty());
        let permission = entity::prelude::Permissions::find_by_id(user_uuid)
            .one(&conn)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(permission.user_role, UserRole::Poet);
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation {
                modifyUserRole(email: \"soon_to_be_poet@test.com\", newUserRole: \"ADMIN\")
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: Uuid::nil(),
                    user_role: UserRole::Moderator,
                }))),
            )
            .await;
        assert_eq!(result.errors[0].message, "UnAuthorized".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation {
                modifyUserRole(email: \"who???@test.com\", newUserRole: \"ADMIN\")
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: Uuid::nil(),
                    user_role: UserRole::Moderator,
                }))),
            )
            .await;
        assert_eq!(
            result.errors[0].message,
            "Can't find user to promote given email.".to_string()
        );
    }
    #[tokio::test]
    #[traced_test]
    async fn invite_user_test() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key, email);
        let uuid = create_login_with_password(&conn, "mod_email@test.com".into(), "1234".into())
            .await
            .unwrap();
        entity::permissions::ActiveModel {
            user_uuid: Set(uuid),
            user_role: Set(UserRole::Moderator),
            ..Default::default()
        }
        .update(&conn)
        .await
        .unwrap();

        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation {
                inviteUser(email: \"sOmE_rAnDoM_EmAiL@test.com\", userRole: \"POET\")
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: uuid,
                    user_role: UserRole::Moderator,
                }))),
            )
            .await;
        eprintln!("{:?}", result.errors);
        assert!(result.errors.is_empty());
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation {
                inviteUser(email: \"sOmE_rAnDoM_EmAiL@test.com\", userRole: \"POET\")
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        assert_eq!(result.errors[0].message, String::from("Unauthorized."));
    }
}
*/