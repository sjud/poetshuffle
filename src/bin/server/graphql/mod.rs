pub mod schema {
    use std::collections::BTreeMap;
    use async_graphql::*;
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
                Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"SELECT user_uuid FROM users
                     WHERE email = $1, password = crypt($2,password)"#,
                    vec![Value::from(email),Value::from(pass)]
                ))
                .one(db)
                .await
                .map_err(|err|format!("{}",err))? {
            // Fetch the permission from the permission table.
            if let Some(permission) = entity::permissions::Entity::find_by_id(user_uuid)
                .one(db)
                .await
                .map_err(|err|format!("{}",err))? {

                // Serialize the permission as the "sub" value of our future token.
                claims.insert("sub",permission);

                // Sign our claims and return functioning JWT.
                Ok(claims.sign_with_key(key)
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



    pub fn schema(conn:DatabaseConnection)
        -> Schema<Query, EmptyMutation, EmptySubscription> {
        // Create our key for signing JWT's.
        let key: Hmac<Sha256> = Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
            .expect("Expecting valid Hmac<Sha256> from slice.");
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(conn)
            .data(key)
            .finish()
    }
}