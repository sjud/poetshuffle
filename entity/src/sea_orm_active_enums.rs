//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use async_graphql::*;
use sea_orm::entity::prelude::*;
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    EnumIter,
    DeriveActiveEnum,
    Enum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "listener")]
    Listener,
    #[sea_orm(string_value = "moderator")]
    Moderator,
    #[sea_orm(string_value = "poet")]
    Poet,
    #[sea_orm(string_value = "super_admin")]
    SuperAdmin,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "set_status")]
pub enum SetStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "published")]
    Published,
}
