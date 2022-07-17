use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}
use sea_orm::sea_query::Iden;
#[derive(Iden)]
pub enum Poems {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Modify logins so that email is unique.
        let table = Table::alter()
            .table(Poems::Table)
            .drop_column(Alias::new("is_approved"))
            .add_column(
                ColumnDef::new(Alias::new("approved"))
                    .default(false)
                    .not_null()
                    .boolean(),
            )
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        Ok(())
    }
}

/*


*/
