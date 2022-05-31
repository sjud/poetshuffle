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
        let table = Table::alter()
            .table(Logins::Table)
            .add_column(ColumnDef::new(Alias::new("last_login"))
                .timestamp_with_time_zone())
            .add_column(ColumnDef::new(Alias::new("is_validated"))
                .boolean()
                .not_null()
                .default(false))
            .add_column(ColumnDef::new(Alias::new("lost_password_hash"))
                .string())
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Logins::Table)
            .drop_column(Alias::new("last_login"))
            .drop_column(Alias::new("is_validated"))
            .drop_column(Alias::new("lost_password_hash"))
            .to_owned();
        manager.exec_stmt(table).await?;

        Ok(())
    }
}

/*


*/
