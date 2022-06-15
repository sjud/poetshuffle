use async_graphql::*;
use sea_orm::QueryFilter;
use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait,
    DatabaseBackend, DatabaseConnection, EntityTrait, Statement,
};
use sea_query::{Expr, PostgresQueryBuilder, Query as SeaQuery};
use sha2::Sha256;

pub mod login;
pub mod sets;
pub mod admin;
pub mod publish;

pub fn time_now() -> DateTimeWithTimeZone {
    use chrono::Utc;
    DateTimeWithTimeZone::from(Utc::now())
}