use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}
use sea_orm::sea_query::Iden;

#[derive(Iden)]
pub enum Invitations {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"ALTER TABLE IF EXISTS invitations
        ADD COLUMN IF NOT EXISTS user_role user_role NOT NULL;"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager
            .get_connection()
            .execute(stmt)
            .await
            .map(|_| ())
            .expect("If this migration doesn't work then invitations won't work.");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Invitations::Table)
            .drop_column(Alias::new("user_role"))
            .to_owned();
        manager.exec_stmt(table).await?;
        Ok(())
    }
}

/*


*/
