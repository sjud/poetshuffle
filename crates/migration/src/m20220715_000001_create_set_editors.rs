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
        let sql = r#"CREATE TABLE IF NOT EXISTS set_editors(
    user_uuid UUID NOT NULL references users(user_uuid),
    set_uuid UUID NOT NULL references sets(set_uuid),
    creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY(user_uuid,set_uuid)
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    Ok(())
    }
}

/*


*/
