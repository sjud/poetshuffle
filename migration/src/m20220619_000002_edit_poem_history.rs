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
        let sql = r#"CREATE TABLE IF NOT EXISTS edit_poem_history(
    user_uuid UUID NOT NULL PRIMARY KEY,
    creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    poem_uuid UUID REFERENCES poems(poem_uuid),
    edit_title VARCHAR,
    edit_link VARCHAR,
    edit_idx INTEGER,
    is_approved BOOLEAN,
    is_deleted BOOLEAN
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::users::Entity)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

/*


*/
