use crate::extension::postgres::Type;
use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}
enum SetStatus {
    Type,
    Pending,
    Published,
}

impl Iden for SetStatus {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "set_status",
                Self::Pending => "pending",
                Self::Published => "published",
            }
        )
        .unwrap();
    }
}
enum UserRole {
    Type,
    Listener,
    Poet,
    Moderator,
    Admin,
    SuperAdmin,
}

impl Iden for UserRole {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "user_role",
                Self::Listener => "listener",
                Self::Poet => "poet",
                Self::Moderator => "moderator",
                Self::Admin => "admin",
                Self::SuperAdmin => "super_admin",
            }
        )
        .unwrap();
    }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO don't Ignore errors because I don't
        // TODO know how to create type if not exists???
        let _ = manager
            .create_type(
                Type::create()
                    .as_enum(SetStatus::Type)
                    .values(vec![SetStatus::Pending, SetStatus::Published])
                    .to_owned(),
            )
            .await;
        let _ = manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Type)
                    .values(vec![
                        UserRole::Listener,
                        UserRole::Poet,
                        UserRole::Moderator,
                        UserRole::Admin,
                        UserRole::SuperAdmin,
                    ])
                    .to_owned(),
            )
            .await;
        let sql = r#"
CREATE EXTENSION pgcrypto;"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        let _ = manager.get_connection().execute(stmt).await.map(|_| ());
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(SetStatus::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(UserRole::Type).to_owned())
            .await?;

        Ok(())
    }
}

/*


*/
