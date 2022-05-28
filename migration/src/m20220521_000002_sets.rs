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
        let sql = r#"CREATE TABLE IF NOT EXISTS sets(
    set_uuid UUID NOT NULL PRIMARY KEY,
    creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    collection_title VARCHAR(100) NOT NULL,
    originator_uuid UUID NOT NULL REFERENCES users(user_uuid),
    set_status set_status NOT NULL,
    collection_link VARCHAR(250) NOT NULL,
    editor_uuid UUID REFERENCES users(user_uuid),
    approved BOOL NOT NULL
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entity::sets::Entity).cascade().to_owned())
            .await
    }
}

/*


 */