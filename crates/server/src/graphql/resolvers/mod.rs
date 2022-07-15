use async_graphql::*;
use sea_orm::{ActiveValue, QueryFilter};
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
// The idea with these two functions is that update resolvers are given
// optional values. Which might be for nullable database fields or non-nullable
// database fields. Each function unpacks the optional update arg and returns
// a Some(value) for nullable or just value for non-nullable.
// Or if input is None doesn't set the field, equivalent to ..Default::default()
pub fn update_value<V: Into<sea_orm::Value>>(
    v: Option<V>,
) -> ActiveValue<V> {
    if let Some(value) = v {
        ActiveValue::set(value)
    } else {
        ActiveValue::not_set()
    }
}
pub fn update_nullable_value<V: Into<sea_orm::Value> + migration::Nullable>(
    v: Option<V>,
) -> ActiveValue<Option<V>> {
    if let Some(value) = v {
        ActiveValue::set(Some(value))
    } else {
        ActiveValue::not_set()
    }
}