//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "edit_poem_history"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub history_uuid: Uuid,
    pub user_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub poem_uuid: Uuid,
    pub edit_banter_uuid: Option<Uuid>,
    pub edit_title: Option<String>,
    pub edit_link: Option<String>,
    pub edit_idx: Option<i32>,
    pub edit_is_approved: Option<bool>,
    pub edit_is_deleted: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    HistoryUuid,
    UserUuid,
    CreationTs,
    PoemUuid,
    EditBanterUuid,
    EditTitle,
    EditLink,
    EditIdx,
    EditIsApproved,
    EditIsDeleted,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    HistoryUuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Banters,
    Poems,
    Users,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::HistoryUuid => ColumnType::Uuid.def(),
            Self::UserUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::PoemUuid => ColumnType::Uuid.def(),
            Self::EditBanterUuid => ColumnType::Uuid.def().null(),
            Self::EditTitle => ColumnType::String(None).def().null(),
            Self::EditLink => ColumnType::String(None).def().null(),
            Self::EditIdx => ColumnType::Integer.def().null(),
            Self::EditIsApproved => ColumnType::Boolean.def().null(),
            Self::EditIsDeleted => ColumnType::Boolean.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Banters => Entity::belongs_to(super::banters::Entity)
                .from(Column::EditBanterUuid)
                .to(super::banters::Column::BanterUuid)
                .into(),
            Self::Poems => Entity::belongs_to(super::poems::Entity)
                .from(Column::PoemUuid)
                .to(super::poems::Column::PoemUuid)
                .into(),
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserUuid)
                .to(super::users::Column::UserUuid)
                .into(),
        }
    }
}

impl Related<super::banters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Banters.def()
    }
}

impl Related<super::poems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poems.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}