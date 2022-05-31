use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseBackend,
    DatabaseConnection, EntityTrait, Statement,
};

use crate::types::iden::Logins;
use anyhow::Result;
use entity::sea_orm_active_enums::UserRole;
use sea_query::{Expr, PostgresQueryBuilder, Query as SeaQuery};

pub async fn populate_db_with_test_data(conn: &DatabaseConnection) -> Result<()> {
    let uuid = Uuid::from_u128(1);
    //drop_db_with_test_data(conn, uuid).await.unwrap();
    let user = entity::users::ActiveModel {
        user_uuid: Set(uuid),
        ..Default::default()
    };
    user.insert(conn).await.unwrap();

    let query = SeaQuery::insert()
        .into_table(Logins::Table)
        .columns(vec![Logins::UserUuid, Logins::Email, Logins::Password])
        .exprs(vec![
            Expr::val(uuid).into(),
            Expr::val("test@test.com".to_string()).into(),
            Expr::cust_with_values("crypt(?, gen_salt('bf'))", vec!["1234".to_string()]),
        ])
        .unwrap()
        .to_owned()
        .to_string(PostgresQueryBuilder);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    conn.execute(stmt).await.unwrap();
    let permission = entity::permissions::ActiveModel {
        user_uuid: Set(uuid),
        user_role: Set(UserRole::Listener),
    };
    permission.insert(conn).await.unwrap();
    Ok(())
}
async fn drop_db_with_test_data(conn: &DatabaseConnection, uuid: Uuid) -> Result<()> {
    entity::logins::Entity::delete(entity::logins::ActiveModel {
        user_uuid: Set(uuid),
        ..Default::default()
    })
    .exec(conn)
    .await
    .unwrap();
    entity::permissions::Entity::delete(entity::permissions::ActiveModel {
        user_uuid: Set(uuid),
        user_role: Set(UserRole::Listener),
    })
    .exec(conn)
    .await
    .unwrap();
    entity::users::Entity::delete(entity::users::ActiveModel {
        user_uuid: Set(uuid),
        ..Default::default()
    })
    .exec(conn)
    .await
    .unwrap();
    Ok(())
}
