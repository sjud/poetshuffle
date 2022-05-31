use super::*;
use hmac::Hmac;
use jwt::SignWithKey;
use std::collections::BTreeMap;

use crate::types::iden::Logins;

#[derive(Default)]
pub struct LoginQuery;
#[derive(Default)]
pub struct LoginMutation;
#[Object]
impl LoginQuery {
    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        pass: String,
    ) -> Result<String> {
        // Initialize constants.
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let key = ctx.data::<Hmac<Sha256>>().unwrap();
        let mut claims = BTreeMap::new();
        // If the password and email match we get back the user uuid.
        if let Some(Ok(user_uuid)) = {
            let query = SeaQuery::select()
                .column(Logins::UserUuid)
                .from(Logins::Table)
                .and_where(Expr::cust_with_values("email = ?", vec![email]).into())
                .and_where(Expr::cust_with_values(
                    "password = crypt(?, password)",
                    vec![pass],
                ))
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
                // Serialize the permission as the "sub" value of our future token.
                claims.insert("sub", permission);

                // Sign our claims and return functioning JWT.
                Ok(claims
                    .sign_with_key(key)?)
            } else {
                Err(anyhow::Error::msg("No matching Permission."))
            }
        } else {
            Err(anyhow::Error::msg("No matching Login."))
        }
    }
}
fn create_login(
    db:&DatabaseConnection,
    email:String,
    password:String,
) -> Result<Uuid> {
    let uuid = Uuid::new_v4();
    let query = SeaQuery::insert()
        .into_table(Logins::Table)
        .columns(vec![Logins::UserUuid, Logins::Email, Logins::Password])
        .exprs(vec![
            Expr::val(uuid).into(),
            Expr::val(email).into(),
            Expr::cust_with_values("crypt(?, gen_salt('bf'))", vec![password]),
        ])?
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    conn.execute(stmt).await?;
    Ok(uuid)
}
#[Object]
impl LoginMutation{
    async fn register(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<String> {
        // check to see if email exists in DB
        // if it does not
        // create lost_password_code
        // email lost_password_code to email
        // store lost_password_hash
        Ok("".into())
    }
    async fn validate_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        new_password: String,
        lost_password_code:String,
    ) -> Result<String> {
        // THIS IS THE SAME AS RESET BUT WITH VALIDATION UPDATE
        // WE MAY DELETE UNVALIDATED USERS WHENEVER CONVENIENT.
        // compare lost_password_code to lost_password_hash
        // by looking it up in conjunction with email
        // if comparison is equal
        // set password to new_password
        // delete lost_password_hash
        // set is_validated to true <- only diff
        // tell user to login with new password.
    }
    async fn change_password(&self,
        ctx:&Context<'_>,
        email: String,
        old_password: String,
        new_password: String,
    ) -> Result<String> {
        // look up email and old password
        // if old password hash matched hashed password in db
        // set the db password to be the hash of the new password
        // return successfully.
        Ok("".into())
    }
    async fn request_reset_password(&self,
        ctx:&Context<'_>,
        email: String,
    ) -> Result<String> {
        // create lost_password_code
        // store lost_password_hash
        // email lost_password_code to the email
        // respond with a string requesting user to check their email.
        Ok("".into())
    }
    async fn reset_password(
        &self,
        ctx:&Context<'_>,
        email: String,
        new_password: String,
        lost_password_code:String,
    ) -> Result<String> {
        // compare lost_password_code to lost_password_hash
        // by looking it up in conjunction with email
        // if comparison is equal
        // set password to new_password
        // delete lost_password_hash
        // tell user to login with new password.
    }
}
#[cfg(test)]
mod test {
    use hmac::digest::KeyInit;
    use super::*;
    use crate::graphql::schema::new_schema;
    use crate::{DATABASE_URL, populate_db_with_test_data};

    #[tokio::test]
    async fn test_login_graphql_query() {
        println!("{:?}",&*DATABASE_URL);
        let key: Hmac<Sha256> = Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
            .expect("Expecting valid Hmac<Sha256> from slice.");

        let conn = sea_orm::Database::connect(&*DATABASE_URL)
            .await
            .expect("Expecting DB connection given DATABASE_URL.");

        populate_db_with_test_data(&conn).await.unwrap();

        let schema = new_schema(conn,key);

        let result = schema
            .execute(
                "{
        login(email: \"test@test.com\", pass: \"1234\")
        }",
            )
            .await;
        assert!(result.errors.is_empty())
    }
}
