pub use sea_orm_migration::prelude::*;

mod m20220521_000000_init;
mod m20220521_000001_create_users;
mod m20220521_000002_sets;
mod m20220521_000003_permissions;
mod m20220521_000004_set_options;
mod m20220521_000005_logins;
mod m20220521_000006_pen_names;
mod m20220521_000007_banters;
mod m20220521_000008_poems;
mod m20220521_000009_intros;
mod m20220521_000010_comments;
mod m20220521_000011_orders;
mod m20220531_000000_alter_logins;
mod m20220531_000001_alter_logins;
mod m20220609_000001_alter_users_logins;
mod m20220614_000001_create_invitations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220521_000000_init::Migration),
            Box::new(m20220521_000001_create_users::Migration),
            Box::new(m20220521_000002_sets::Migration),
            Box::new(m20220521_000003_permissions::Migration),
            Box::new(m20220521_000004_set_options::Migration),
            Box::new(m20220521_000005_logins::Migration),
            Box::new(m20220521_000006_pen_names::Migration),
            Box::new(m20220521_000007_banters::Migration),
            Box::new(m20220521_000008_poems::Migration),
            Box::new(m20220521_000009_intros::Migration),
            Box::new(m20220521_000010_comments::Migration),
            Box::new(m20220521_000011_orders::Migration),
            Box::new(m20220531_000000_alter_logins::Migration),
            Box::new(m20220531_000001_alter_logins::Migration),
            Box::new(m20220609_000001_alter_users_logins::Migration),
            Box::new(m20220614_000001_create_invitations::Migration),

        ]
    }
}
