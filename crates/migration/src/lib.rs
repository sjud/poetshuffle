pub use sea_orm_migration::prelude::*;

mod m20220521_000000_init;
mod m20220521_000001_create_users;
mod m20220521_000002_sets;
mod m20220521_000003_permissions;
mod m20220521_000005_logins;
mod m20220521_000006_pen_names;
mod m20220521_000007_banters;
mod m20220521_000008_poems;
mod m20220521_000009_intros;
mod m20220521_000010_comments;
mod m20220531_000000_alter_logins;
mod m20220531_000001_alter_logins;
mod m20220609_000001_alter_users_logins;
mod m20220614_000001_create_invitations;
mod m20220615_000001_alter_invitations;
mod m20220619_000001_alter_poems;
mod m20220619_000004_alter_sets;
mod m20220715_000001_create_set_editors;
mod m20220715_000002_alter_sets;
mod m20220715_000003_alter_poems;
mod m20220715_000004_alter_poems;
mod m20220716_000001_alter_poems;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220521_000000_init::Migration),
            Box::new(m20220521_000001_create_users::Migration),
            Box::new(m20220521_000002_sets::Migration),
            Box::new(m20220521_000003_permissions::Migration),
            Box::new(m20220521_000005_logins::Migration),
            Box::new(m20220521_000006_pen_names::Migration),
            Box::new(m20220521_000007_banters::Migration),
            Box::new(m20220521_000008_poems::Migration),
            Box::new(m20220521_000009_intros::Migration),
            Box::new(m20220521_000010_comments::Migration),
            Box::new(m20220531_000000_alter_logins::Migration),
            Box::new(m20220531_000001_alter_logins::Migration),
            Box::new(m20220609_000001_alter_users_logins::Migration),
            Box::new(m20220614_000001_create_invitations::Migration),
            Box::new(m20220615_000001_alter_invitations::Migration),
            Box::new(m20220619_000001_alter_poems::Migration),
            Box::new(m20220619_000004_alter_sets::Migration),
            Box::new(m20220715_000001_create_set_editors::Migration),
            Box::new(m20220715_000002_alter_sets::Migration),
            Box::new(m20220715_000003_alter_poems::Migration),
            Box::new(m20220715_000004_alter_poems::Migration),
            Box::new(m20220716_000001_alter_poems::Migration),



        ]
    }
}
