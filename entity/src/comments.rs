//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "comments"
    }
}
use async_graphql::*;
#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, SimpleObject)]
#[graphql(concrete(name = "Comment", params()))]
pub struct Model {
    pub comment_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub poem_uuid: Uuid,
    pub originator_uuid: Uuid,
    pub set_uuid: Uuid,
    pub body: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CommentUuid,
    CreationTs,
    PoemUuid,
    OriginatorUuid,
    SetUuid,
    Body,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CommentUuid,
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
    Poems,
    Sets,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CommentUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::PoemUuid => ColumnType::Uuid.def(),
            Self::OriginatorUuid => ColumnType::Uuid.def(),
            Self::SetUuid => ColumnType::Uuid.def(),
            Self::Body => ColumnType::String(Some(200u32)).def(),
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
            Self::Poems => Entity::belongs_to(super::poems::Entity)
                .from(Column::PoemUuid)
                .to(super::poems::Column::PoemUuid)
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

impl Related<super::poems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poems.def()
    }
}

impl Related<super::sets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
