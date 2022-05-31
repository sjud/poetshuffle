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
#[Object]
impl LoginMutation{
    async fn register(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<String> {
        Ok("".into())
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
