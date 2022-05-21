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
        let sql = r#"CREATE TABLE IF NOT EXISTS comments(
        comment_uuid UUID NOT NULL PRIMARY KEY,
        creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
        poem_uuid UUID NOT NULL REFERENCES poems(poem_uuid),
        originator_uuid UUID NOT NULL REFERENCES users(user_uuid),
        set_uuid UUID REFERENCES sets(set_uuid) NOT NULL,
        body VARCHAR(200) NOT NULL
);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entity::comments::Entity).cascade().to_owned())
            .await
    }
}

/*


 */