//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "banters"
    }
}
use async_graphql::*;
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, SimpleObject)]
#[graphql(concrete(name = "Banter", params()))]
pub struct Model {
    pub banter_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub originator_uuid: Uuid,
    pub editor_uuid: Option<Uuid>,
    pub approved: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    BanterUuid,
    CreationTs,
    OriginatorUuid,
    EditorUuid,
    Approved,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    BanterUuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users2,
    Users1,
    Poems,
    EditPoemHistory,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::BanterUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::OriginatorUuid => ColumnType::Uuid.def(),
            Self::EditorUuid => ColumnType::Uuid.def().null(),
            Self::Approved => ColumnType::Boolean.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users2 => Entity::belongs_to(super::users::Entity)
                .from(Column::EditorUuid)
                .to(super::users::Column::UserUuid)
                .into(),
            Self::Users1 => Entity::belongs_to(super::users::Entity)
                .from(Column::OriginatorUuid)
                .to(super::users::Column::UserUuid)
                .into(),
            Self::Poems => Entity::has_many(super::poems::Entity).into(),
            Self::EditPoemHistory => Entity::has_many(super::edit_poem_history::Entity).into(),
        }
    }
}

impl Related<super::poems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poems.def()
    }
}

impl Related<super::edit_poem_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EditPoemHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
