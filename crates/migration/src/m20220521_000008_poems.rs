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
        let sql = r#"CREATE TABLE IF NOT EXISTS poems(
    poem_uuid UUID PRIMARY KEY,
    originator_uuid UUID REFERENCES users(user_uuid) NOT NULL,
    creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    set_uuid UUID REFERENCES sets(set_uuid) NOT NULL,
    banter_uuid UUID REFERENCES banters(banter_uuid),
    title VARCHAR(100) NOT NULL,
    idx INTEGER NOT NULL,
    part_of_poetshuffle BOOL NOT NULL,
    approved BOOL);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::poems::Entity)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

/*


*/
