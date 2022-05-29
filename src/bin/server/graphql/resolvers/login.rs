use jwt::SignWithKey;
use hmac::Hmac;
use anyhow::Result;
use std::collections::BTreeMap;
use super::*;


use crate::{types::iden::Logins};

#[derive(Default)]
pub struct LoginQuery;
#[Object]
impl LoginQuery {
    async fn login(
        &self,
        ctx:&Context<'_>,
        email:String,
        pass:String,
    ) -> Result<String,String> {
        // Initialize constants.
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let key = ctx.data::<Hmac<Sha256>>().unwrap();
        let mut claims = BTreeMap::new();
        // If the password and email match we get back the user uuid.
        if let Some(Ok(user_uuid)) = {
            let query = SeaQuery::select()
                .column(Logins::UserUuid)
                .from(Logins::Table)
                .and_where(
                    Expr::cust_with_values("email = ?",vec![email]).into())
                .and_where(
                    Expr::cust_with_values("password = crypt(?, password)",vec![pass]))
                .to_owned()
                .to_string(PostgresQueryBuilder);
            let stmt = Statement::from_string(DatabaseBackend::Postgres,
                                              query);
            db.query_one(stmt)
                .await
                .map(|option|
                    option.map(|item|
                        item.try_get("","user_uuid")))
                .map_err(|err|format!("{}",err))?
        } {
            // Fetch the permission from the permission table using user_uuid.
            if let Some(permission) = entity::permissions::Entity::find_by_id(user_uuid)
                .one(db)
                .await
                .map_err(|err|format!("{}",err))? {

                // Serialize the permission as the "sub" value of our future token.
                claims.insert("sub",permission);

                // Sign our claims and return functioning JWT.
                Ok(
                    claims.sign_with_key(key)
                        .map_err(|err|format!("{}",err))?
                )
            } else {
                Err(String::from("No matching Permission."))
            }
        } else {
            Err(String::from("No matching Login."))
        }
    }
}
#[cfg(test)]
mod test {
    use crate::graphql::schema::new_schema;
    use crate::populate_db_with_test_data;
    use super::*;


    #[tokio::test]
    async fn test_login_graphql_query() {
        let conn = sea_orm::Database::connect(&*DATABASE_URL).await
            .expect("Expecting DB connection given DATABASE_URL.");
        populate_db_with_test_data(&conn).await.unwrap();
        let schema = new_schema(conn);

        let result = schema.execute("{
        login(email: \"test@test.com\", pass: \"1234\")
        }")
            .await;
        assert!(result.errors.is_empty())

    }
}