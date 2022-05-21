use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;
use entity::users::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"CREATE TABLE IF NOT EXISTS intros(
        set_uuid UUID NOT NULL PRIMARY KEY REFERENCES sets(set_uuid),
        creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
        originator_uuid UUID NOT NULL REFERENCES users(user_uuid),
        editor_uuid UUID NOT NULL REFERENCES users(user_uuid),
        approved BOOL
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entity::intros::Entity).cascade().to_owned())
            .await
    }
}

/*


 */