use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}
use sea_orm::sea_query::Iden;
#[derive(Iden)]
pub enum Sets {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Modify logins so that email is unique.
        let table = Table::alter()
            .table(Sets::Table)
            .drop_column(Alias::new("approved"))
            .drop_column(Alias::new("collection_title"))
            .drop_column(Alias::new("collection_link"))
            .add_column(
                ColumnDef::new(Alias::new("title"))
                    .default("")
                    .not_null()
                    .string(),
            )
            .add_column(
                ColumnDef::new(Alias::new("link"))
                    .default("")
                    .not_null()
                    .string(),
            )
            .add_column(
                ColumnDef::new(Alias::new("is_approved"))
                    .default(false)
                    .not_null()
                    .boolean(),
            )
            .add_column(
                ColumnDef::new(Alias::new("is_deleted"))
                    .default(false)
                    .not_null()
                    .boolean(),
            )
            .add_column(ColumnDef::new(Alias::new("last_edit_ts")).timestamp())
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Sets::Table)
            .add_column(
                ColumnDef::new(Alias::new("approved"))
                    .default(false)
                    .boolean(),
            )
            .add_column(
                ColumnDef::new(Alias::new("collection_title"))
                    .default("")
                    .not_null()
                    .string(),
            )
            .add_column(
                ColumnDef::new(Alias::new("collection_link"))
                    .default("")
                    .not_null()
                    .string(),
            )
            .drop_column(Alias::new("title"))
            .drop_column(Alias::new("link"))
            .drop_column(Alias::new("is_approved"))
            .drop_column(Alias::new("is_deleted"))
            .drop_column(Alias::new("last_edit_ts"))
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }
}

/*


*/
