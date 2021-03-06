//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "intros"
    }
}

use async_graphql::*;
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, SimpleObject)]
#[graphql(concrete(name = "Intro", params()))]
pub struct Model {
    pub intro_uuid: Uuid,
    pub set_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub originator_uuid: Uuid,
    pub approved: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    IntroUuid,
    SetUuid,
    CreationTs,
    OriginatorUuid,
    Approved,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    IntroUuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Sets,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::IntroUuid => ColumnType::Uuid.def(),
            Self::SetUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::OriginatorUuid => ColumnType::Uuid.def(),
            Self::Approved => ColumnType::Boolean.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::OriginatorUuid)
                .to(super::users::Column::UserUuid)
                .into(),
            Self::Sets => Entity::belongs_to(super::sets::Entity)
                .from(Column::SetUuid)
                .to(super::sets::Column::SetUuid)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::sets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
