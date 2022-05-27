
use std::collections::BTreeMap;
use async_graphql::*;
use async_graphql::extensions::Tracing;
use hmac::digest::KeyInit;
use hmac::Hmac;
use sea_orm::{Value,DatabaseConnection,EntityTrait,DbBackend, DbErr, Statement};
use sea_orm::prelude::Uuid;
use jwt::SignWithKey;
use sha2::Sha256;

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
        if let Some(entity::users::Model{user_uuid, .. }) = entity::users::Entity::find().
            from_raw_sql(
                // TODO IS THIS VULNERABLE TO SQL INJECTION ATTACKS ???
                Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"SELECT user_uuid FROM logins
                     WHERE email = $1 AND password = crypt($2,password)"#,
                    vec![Value::from(email),Value::from(pass)]
                ))
            .one(db)
            .await
            .map_err(|err|format!("{}",err))? {
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
            Err(String::from("No matching User."))
        }
    }
}

#[derive(MergedObject, Default)]
pub struct Query(LoginQuery);
pub type PoetShuffleSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds our Schema for our service layer using DB Conn.
/// It generates internally a JWT key by using the env var JWT_SECRET.
pub fn new_schema(conn:DatabaseConnection)
                  ->  PoetShuffleSchema {
    // Create our key for signing JWT's.
    let key: Hmac<Sha256> = Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
        .expect("Expecting valid Hmac<Sha256> from slice.");
    // Build our schema from our merged top level queries, and add
    // a database conneciton and JWT key.
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(conn)
        .data(key)
        // Tracing extension logs query info at the INFO level.
        .extension(Tracing)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn
}