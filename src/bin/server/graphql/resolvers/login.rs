use super::*;
use hmac::Hmac;
use jwt::SignWithKey;
use std::collections::BTreeMap;
use chrono::Utc;
use passwords::PasswordGenerator;
use postmark::reqwest::PostmarkClient;
use sea_orm::IntoSimpleExpr;
use sea_query::SimpleExpr;
use sea_query::value::Nullable;
use entity::logins::ActiveModel as LoginsActiveModel;
use entity::prelude::Logins as LoginsEntity;
use sea_orm::QuerySelect;
use entity::sea_orm_active_enums::UserRole;

use crate::{OUTBOUND_EMAIL, URL_BASE};
use crate::email::{Email, Postmark};

#[cfg(test)]
use crate::email::MockEmail;

#[derive(Default)]
pub struct LoginMutation;
#[derive(Iden)]
pub enum Logins{
    Table,
    UserUuid,
    Email,
    Password,
    LostPasswordHash,
}

fn new_lost_password_code() -> Result<String> {
    passwords::PasswordGenerator::new().length(32).generate_one()
        .map_err(|err|err.into())
}

async fn create_login_with_password(
    db:&DatabaseConnection,
    email:String,
    password:String,
) -> Result<Uuid> {
    let uuid = Uuid::new_v4();
    // Create user
    entity::users::ActiveModel{
        user_uuid:Set(uuid),
        ..Default::default()
    }.insert(db).await?;
    // Create default permission

    entity::permissions::ActiveModel{
        user_uuid:Set(uuid),
        user_role:Set(UserRole::Listener),
        ..Default::default()
    }.insert(db).await?;

    // Create login
    let query = SeaQuery::insert()
        .into_table(Logins::Table)
        .columns(vec![Logins::UserUuid, Logins::Email, Logins::Password])
        .exprs(vec![
            Expr::val(uuid).into(),
            Expr::val(email).into(),
            Expr::cust_with_values("crypt($1, gen_salt('bf'))", vec![password]).into(),
        ])?
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    db.execute(stmt).await?;
    Ok(uuid)
}

async fn create_login_with_lost_password_code(
    db:&DatabaseConnection,
    email:String,
    lost_password_code:String,
) -> Result<Uuid> {
    let uuid = Uuid::new_v4();
    // Create user
    entity::users::ActiveModel{
        user_uuid:Set(uuid),
        ..Default::default()
    }.insert(db).await?;
    // Create default permission

    entity::permissions::ActiveModel{
        user_uuid:Set(uuid),
        user_role:Set(UserRole::Listener),
        ..Default::default()
    }.insert(db).await?;

    let query = SeaQuery::insert()
        .into_table(Logins::Table)
        .columns(vec![Logins::UserUuid, Logins::Email,Logins::Password, Logins::LostPasswordHash])
        .exprs(vec![
            Expr::val(uuid).into(),
            Expr::val(email).into(),
            Expr::cust_with_values("crypt($1, gen_salt('bf'))", vec![
                passwords::PasswordGenerator::new().length(16).generate_one()?
            ]),
            Expr::cust_with_values("crypt($1, gen_salt('bf'))", vec![lost_password_code]),
        ])?
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    db.execute(stmt).await?;
    Ok(uuid)
}

async fn update_login_with_password_given_lost_password_code(
    db:&DatabaseConnection,
    email:String,
    password:String,
    lost_password_code:String,
) -> Result<Uuid> {
    let query = SeaQuery::update()
        .table(Logins::Table)
        .col_expr(Logins::Password,
                  Expr::cust_with_values("crypt($1, gen_salt('bf'))", vec![password]))
        .col_expr(Logins::LostPasswordHash,SimpleExpr::Value(String::null()))
        .and_where(Expr::col(Logins::Email).eq(email))
        .and_where(Expr::cust_with_values(
            "lost_password_hash = crypt($1, lost_password_hash)",
            vec![lost_password_code],
        ))
        .returning_col(Logins::UserUuid)
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    if let Some(result) = db.query_one(stmt).await? {
        result.try_get("","user_uuid")
            .map_err(|err|err.into())
    } else {
        Err("Internal Server Error.".into())
    }
}

async fn update_login_with_lost_password_code(
    db:&DatabaseConnection,
    email:String,
    lost_password_code:String
) -> Result<Uuid> {
    let query = SeaQuery::update()
        .table(Logins::Table)
        .col_expr(Logins::LostPasswordHash,
                  Expr::cust_with_values(
                      "lost_password_hash = crypt($1, lost_password_hash)",
                      vec![lost_password_code],
                  ))
        .and_where(Expr::col(Logins::Email).eq(email))
        .returning_col(Logins::UserUuid)
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    if let Some(result) = db.query_one(stmt).await? {
        result.try_get("","user_uuid")
            .map_err(|err|err.into())
    } else {
        Err("User doesn't exist.".into())
    }
}

async fn update_login_with_new_password_given_current_password(
    db:&DatabaseConnection,
    email:String,
    new_password:String,
    current_password:String,
) -> Result<Uuid> {
    let query = SeaQuery::update()
        .table(Logins::Table)
        .col_expr(Logins::Password,
                  Expr::cust_with_values("crypt($1, gen_salt('bf'))", vec![new_password]))
        .and_where(Expr::col(Logins::Email).eq(email))
        .and_where(Expr::cust_with_values(
            "password = crypt($1, password)",
            vec![current_password],
        ))
        .returning_col(Logins::UserUuid)
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    if let Some(result) = db.query_one(stmt).await? {
        result.try_get("","user_uuid")
            .map_err(|err|err.into())
    } else {
        Err("Internal Server Error.".into())
    }
}

async fn find_login_by_email(db:&DatabaseConnection,email:&str)->Result<Option<entity::logins::Model>>{
    LoginsEntity::find()
        .having(entity::logins::Column::Email.eq(email))
        .group_by(entity::logins::Column::UserUuid)
        .one(db)
        .await
        .map_err(|err|{
            println!("{:?}",err);
            err.into()})
}


#[Object]
impl LoginMutation{
    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        pass: String,
    ) -> Result<String> {
        // Initialize constants.
        let db = ctx.data::<DatabaseConnection>()?;
        let key = ctx.data::<Hmac<Sha256>>()?;
        let mut claims = BTreeMap::new();
        // If the password and email match we get back the user uuid.
        if let Some(Ok(user_uuid)) = {
            let query = SeaQuery::select()
                .column(Logins::UserUuid)
                .from(Logins::Table)
                .and_where(Expr::cust_with_values(
                    "email = $1", vec![email]).into())
                .and_where(Expr::cust_with_values(
                    "password = crypt($1, password)", vec![pass]))
                .to_owned()
                .to_string(PostgresQueryBuilder);
            let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
            db.query_one(stmt)
                .await
                .map(|option| option.map(|item| item.try_get("", "user_uuid")))?
        } {
            // Fetch the permission from the permission table using user_uuid.
            if let Some(permission) = entity::permissions::Entity::find_by_id(user_uuid)
                .one(db)
                .await?
            {
                // update last login before we send the permissions
                LoginsActiveModel{
                    user_uuid:Set(user_uuid),
                    last_login:Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
                    ..Default::default()
                }.update(db).await?;

                // Serialize the permission as the "sub" value of our future token.
                claims.insert("sub", permission);

                // Sign our claims and return functioning JWT.
                Ok(claims
                    .sign_with_key(key)?)
            } else {
                Err(Error::new("No matching Permission."))
            }
        } else {
            Err(Error::new("No matching Login."))
        }
    }
    /// check to see if email exists in DB
    /// if it does not
    /// create lost_password_code
    /// email lost_password_code to email
    /// store lost_password_hash
    async fn register(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<String> {
        #[cfg(test)]
            let email_client = ctx.data::<MockEmail>()?;
        #[cfg(not(test))]
            let email_client = ctx.data::<Postmark>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        let lost_password_code = new_lost_password_code()?;
        if let Some(login) = find_login_by_email(db,&email).await? {
            if let Some(user) = entity::prelude::Users::find_by_id(login.user_uuid).one(db).await? {
                if user.is_validated {
                    return Ok("Already registered, please log in with your password.".into());
                } else {
                    email_client.register(email, lost_password_code).await?;
                }
            } else {
                // This should be impossible...
                return Err("Internal Server Error: Valid login without valid user.".into())
            }
        } else {
            create_login_with_lost_password_code(
                db,
                email.clone(),
                lost_password_code.clone()).await?;
            email_client.register(email,lost_password_code).await?;
        }
        Ok("Please check your email for a validation link.".into())

    }
    /// THIS IS THE SAME AS RESET BUT WITH VALIDATION UPDATE
    /// WE MAY DELETE UNVALIDATED USERS WHENEVER CONVENIENT.
    /// compare lost_password_code to lost_password_hash
    /// by looking it up in conjunction with email
    /// if comparison is equal
    /// set password to new_password
    /// delete lost_password_hash
    /// set is_validated to true <- only diff
    /// tell user to login with new password.
    async fn validate_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        new_password: String,
        lost_password_code:String,
    ) -> Result<String> {

        let db = ctx.data::<DatabaseConnection>()?;
        let uuid = update_login_with_password_given_lost_password_code(
            db,
            email,
            new_password,
            lost_password_code,
        ).await?;
        entity::users::ActiveModel{
            user_uuid:Set(uuid),
            is_validated:Set(true),
            ..Default::default()
        }.update(db).await?;
        Ok("Account validated. Please use your new password to log in.".into())

    }
    /// look up email and old password
    /// if old password hash matched hashed password in db
    /// set the db password to be the hash of the new password
    /// return successfully.
    async fn change_password(&self,
        ctx:&Context<'_>,
        email: String,
        old_password: String,
        new_password: String,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let _ = update_login_with_new_password_given_current_password(
            db,
            email,
            new_password,
            old_password
        ).await?;
        Ok("Password has been updated.".into())
    }
    /// create lost_password_code
    /// store lost_password_hash
    /// email lost_password_code to the email
    /// respond with a string requesting user to check their email.
    async fn request_reset_password(&self,
        ctx:&Context<'_>,
        email: String,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        #[cfg(test)]
        let email_client = ctx.data::<MockEmail>()?;
        #[cfg(not(test))]
            let email_client = ctx.data::<Postmark>()?;

        let lost_password_code = new_lost_password_code()?;
        let _ = update_login_with_lost_password_code(
            db,
            email.clone(),
            lost_password_code.clone()).await?;
        email_client.reset_password(email,lost_password_code).await?;
        Ok("Please check your email for a validation link.".into())
    }
    /// compare lost_password_code to lost_password_hash
    /// by looking it up in conjunction with email
    /// if comparison is equal
    /// set password to new_password
    /// delete lost_password_hash
    /// tell user to login with new password.
    async fn reset_password(
        &self,
        ctx:&Context<'_>,
        email: String,
        new_password: String,
        lost_password_code:String,
    ) -> Result<String> {

        let db = ctx.data::<DatabaseConnection>()?;
        let _ = update_login_with_password_given_lost_password_code(
            db,
            email,
            new_password,
            lost_password_code,
        ).await?;
        Ok("You may now login with your new password.".into())

    }
}

#[cfg(test)]
mod test {
    use hmac::digest::KeyInit;
    use super::*;
    use crate::graphql::schema::new_schema;
    use crate::{DATABASE_URL};
    use crate::email::MockEmail;

    async fn key_conn_email() -> (Hmac<Sha256>,DatabaseConnection, impl Email) {
        (
            Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
                .expect("Expecting valid Hmac<Sha256> from slice."),
            sea_orm::Database::connect(&*DATABASE_URL)
                .await
                .expect("Expecting DB connection given DATABASE_URL."),
            {
                let mut email = MockEmail::new();
                email.expect_register()
                    .returning(|_,_|Ok(()));
                email.expect_reset_password()
                    .returning(|_,_|Ok(()));
                email
            }
            )
    }
    #[tokio::test]
    async fn test_login_graphql_query() {
        let (key,conn,email) = key_conn_email().await;

        create_login_with_password(
            &conn,
            "test@test.com".into(),
            "1234".into())
            .await
            .unwrap();

        let schema = new_schema(conn,key,email);

        let result = schema
            .execute(
                "mutation {
                login(email: \"test@test.com\", pass: \"1234\")
                }",
            )
            .await;
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());
        //assert_eq!(result.data.to_string(),
        //           "{login: \"SOME JWT STRING\"}".to_string()); <- how to test valid jwt?
        // Our return value is some token, let's test is later in an integration test.
        let result = schema
            .execute(
                "mutation {
                login(email: \"test@test.com\", pass: \"12345\")
                }",
            )
            .await;
        assert!(!result.errors.is_empty()); // Should error.
    }

    #[tokio::test]
    async fn test_register_query() {
        let (key,conn,email) = key_conn_email().await;
        let schema = new_schema(conn,key,email);

        let result = schema
            .execute(
                "mutation {
                register(email: \"test1@test.com\")
                }",
            )
            .await;
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());
        assert_eq!(result.data.to_string(),
                   "{register: \"Please check your email for a validation link.\"}".to_string());
        // If we do it again
        let result = schema
            .execute(
                "mutation {
                register(email: \"test1@test.com\")
                }",
            )
            .await;
        // We should get the same result, as we just send a new email and have the same response.
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());
        assert_eq!(result.data.to_string(),
                   "{register: \"Please check your email for a validation link.\"}".to_string());
        // We aren't actually testing to see if our postmark client sends emails so expect bugs.
    }

    #[tokio::test]
    async fn test_validate_user() {
        let (key,conn,email) = key_conn_email().await;
        create_login_with_lost_password_code(
            &conn,
            "test2@test.com".into(),
            "LOSTPASSWORDCODE".into())
            .await
            .unwrap();
        let schema = new_schema(conn.clone(),key,email);
        let result = schema
            .execute(
                "mutation {
                validateUser(email: \"test2@test.com\", newPassword:\"1234\",\
                lostPasswordCode:\"LOSTPASSWORDCODE\")
                }",
            )
            .await;
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());
        assert_eq!(result.data.to_string(),
                   "{validateUser: \"Account validated. Please use your new password to log in.\"}".to_string());
        let login = find_login_by_email(&conn,"test2@test.com")
            .await
            .unwrap()
            .unwrap();
        let user = entity::prelude::Users::find_by_id(login.user_uuid).one(&conn).await
            .unwrap()
            .unwrap();
        assert_eq!(user.is_validated,true);

    }

    #[tokio::test]
    async fn test_change_password() {
        let (key,conn,email) = key_conn_email().await;
        create_login_with_password(&conn,"test4@test.com".into(),"1234".into())
            .await.unwrap();
        let schema = new_schema(conn.clone(),key,email);
        // Change password
        let result = schema
            .execute(
                "mutation {
                changePassword(email: \"test4@test.com\", oldPassword:\"1234\",\
                newPassword:\"12345\")
                }",
            )
            .await;
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());
        assert_eq!(result.data.to_string(),
                   "{changePassword: \"Password has been updated.\"}".to_string());
        // Then log in with new password.
        let result = schema
            .execute(
                "mutation {
                login(email: \"test4@test.com\", pass: \"12345\")
                }",
            )
            .await;
        eprintln!("{:?}",result.errors);
        assert!(result.errors.is_empty());

    }


}
