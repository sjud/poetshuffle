use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"CREATE TABLE IF NOT EXISTS orders(
        order_uuid UUID NOT NULL PRIMARY KEY,
        creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
        purchasee_uuid UUID NOT NULL REFERENCES users(user_uuid),
        set_uuid UUID NOT NULL REFERENCES sets(set_uuid),
        sent_to_address VARCHAR NOT NULL,
        gift BOOL NOT NULL,
        email_delivery_confirmation BOOL NOT NULL,
        purchase_price INTEGER NOT NULL
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::orders::Entity)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

/*


*/
