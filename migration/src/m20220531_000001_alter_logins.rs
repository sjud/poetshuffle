use sea_orm::{ConnectionTrait, Statement, TryFromU64};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}
use sea_orm::sea_query::Iden;
#[derive(Iden)]
pub enum Logins {
    Table,
    UserUuid,
    Email,
    Password,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Modify logins so that email is unique.
        let table = Table::alter()
            .table(Logins::Table)
            .modify_column(ColumnDef::new(Alias::new("email"))
                .not_null()
                .string()
                .unique_key())
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Logins::Table)
            .modify_column(ColumnDef::new(Alias::new("email"))
                .not_null()
                .string())
            .to_owned();
        manager.exec_stmt(table).await?;

        Ok(())
    }
}

/*


*/
