//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub user_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub promoter_uuid: Option<Uuid>,
    pub is_validated: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    UserUuid,
    CreationTs,
    PromoterUuid,
    IsValidated,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    UserUuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SelfRef,
    Permissions,
    PenNames,
    Comments,
    Orders,
    Logins,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::UserUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::PromoterUuid => ColumnType::Uuid.def().null(),
            Self::IsValidated => ColumnType::Boolean.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::SelfRef => Entity::belongs_to(Entity)
                .from(Column::PromoterUuid)
                .to(Column::UserUuid)
                .into(),
            Self::Permissions => Entity::has_many(super::permissions::Entity).into(),
            Self::PenNames => Entity::has_many(super::pen_names::Entity).into(),
            Self::Comments => Entity::has_many(super::comments::Entity).into(),
            Self::Orders => Entity::has_many(super::orders::Entity).into(),
            Self::Logins => Entity::has_many(super::logins::Entity).into(),
        }
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permissions.def()
    }
}

impl Related<super::pen_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PenNames.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::logins::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logins.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}