
use std::collections::BTreeMap;
use async_graphql::*;
use async_graphql::extensions::Tracing;
use hmac::digest::KeyInit;
use hmac::Hmac;
use sea_orm::{Value,DatabaseConnection,EntityTrait,DbBackend, DbErr, Statement};
use sea_orm::prelude::Uuid;
use sea_orm::sea_query::{Expr,Iden,Query as SeaQuery};
use jwt::SignWithKey;
use sha2::Sha256;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_query::PostgresQueryBuilder;
use sea_orm::{ConnectionTrait, DatabaseBackend, Set};

#[derive(Default)]
pub struct LoginQuery;
#[derive(Iden)]
enum Logins{
    Table,
    UserUuid,
    Email,
    Password,
}
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
mod test {
    use sea_orm::{ActiveModelTrait,EntityTrait};
    use crate::DATABASE_URL;
    use anyhow::Result;
    use entity::sea_orm_active_enums::UserRole;
    use super::*;

    async fn populate_db_with_test_data(conn:&DatabaseConnection) -> Result<()> {
        let uuid = Uuid::from_u128(1);
        let _await_here = drop_db_with_test_data(conn,uuid).await.unwrap();
        let user = entity::users::ActiveModel{
            user_uuid:Set(uuid),
            ..Default::default()
        };
        user.insert(conn).await.unwrap();

        let query = SeaQuery::insert()
            .into_table(Logins::Table)
            .columns(vec![Logins::UserUuid,Logins::Email,Logins::Password])
            .exprs(vec![
                Expr::val(uuid).into(),
                Expr::val("test@test.com".to_string()).into(),
                Expr::cust_with_values(
                    "crypt(?, gen_salt('bf'))",vec!["1234".to_string()]
                ),
            ]).unwrap()
            .to_owned()
            .to_string(PostgresQueryBuilder);
        let stmt = Statement::from_string(DatabaseBackend::Postgres,query);
        conn.execute(stmt).await.unwrap();
        let permission = entity::permissions::ActiveModel{
            user_uuid:Set(uuid),
            user_role:Set(UserRole::Listener)
        };
        permission.insert(conn).await.unwrap();
        Ok(())
    }
    async fn drop_db_with_test_data(conn:&DatabaseConnection,uuid:Uuid) -> Result<()> {
        entity::logins::Entity::delete(entity::logins::ActiveModel{
            user_uuid:Set(uuid),
            ..Default::default()
        }).exec(conn).await.unwrap();
        entity::permissions::Entity::delete(entity::permissions::ActiveModel{
            user_uuid:Set(uuid),
            user_role:Set(UserRole::Listener)
        }).exec(conn).await.unwrap();
        entity::users::Entity::delete(entity::users::ActiveModel{
            user_uuid:Set(uuid),
            ..Default::default()
        }).exec(conn).await.unwrap();
        Ok(())
    }
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
        println!("{:?}",result);
        panic!();

    }
}