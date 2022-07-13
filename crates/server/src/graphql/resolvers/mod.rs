use async_graphql::*;
use sea_orm::QueryFilter;
use sea_orm::{
    entity::prelude::*, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait,
    DatabaseBackend, DatabaseConnection, EntityTrait, QuerySelect, Statement,
};

use crate::types::auth::Auth;
use sea_query::{Expr, PostgresQueryBuilder, Query as SeaQuery};
use sha2::Sha256;

pub mod admin;
pub mod login;
pub mod poems;
pub mod publish;
pub mod sets;
pub mod banters;

pub fn time_now() -> DateTimeWithTimeZone {
    use chrono::Utc;
    DateTimeWithTimeZone::from(Utc::now())
}
