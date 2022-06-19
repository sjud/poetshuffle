//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use super::sea_orm_active_enums::SetStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sets"
    }
}

use async_graphql::*;
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, SimpleObject)]
#[graphql(name = "Set")]
pub struct Model {
    pub set_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub originator_uuid: Uuid,
    pub set_status: SetStatus,
    pub editor_uuid: Option<Uuid>,
    pub title: String,
    pub link: String,
    pub is_approved: bool,
    pub is_deleted: bool,
    pub last_edit_ts: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    SetUuid,
    CreationTs,
    OriginatorUuid,
    SetStatus,
    EditorUuid,
    Title,
    Link,
    IsApproved,
    IsDeleted,
    LastEditTs,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    SetUuid,
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
    Intros,
    Comments,
    Poems,
    Orders,
    EditSetHistory,
    SetOptions,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::SetUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::OriginatorUuid => ColumnType::Uuid.def(),
            Self::SetStatus => SetStatus::db_type(),
            Self::EditorUuid => ColumnType::Uuid.def().null(),
            Self::Title => ColumnType::String(None).def(),
            Self::Link => ColumnType::String(None).def(),
            Self::IsApproved => ColumnType::Boolean.def(),
            Self::IsDeleted => ColumnType::Boolean.def(),
            Self::LastEditTs => ColumnType::Timestamp.def().null(),
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
            Self::Intros => Entity::has_many(super::intros::Entity).into(),
            Self::Comments => Entity::has_many(super::comments::Entity).into(),
            Self::Poems => Entity::has_many(super::poems::Entity).into(),
            Self::Orders => Entity::has_many(super::orders::Entity).into(),
            Self::EditSetHistory => Entity::has_many(super::edit_set_history::Entity).into(),
            Self::SetOptions => Entity::has_many(super::set_options::Entity).into(),
        }
    }
}

impl Related<super::intros::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Intros.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::poems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poems.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::edit_set_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EditSetHistory.def()
    }
}

impl Related<super::set_options::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SetOptions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
