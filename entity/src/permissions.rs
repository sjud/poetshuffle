//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use super::sea_orm_active_enums::UserRole;
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "permissions"
    }
}

use async_graphql::*;
#[derive(Clone, Debug, PartialEq,serde_derive::Serialize,serde_derive::Deserialize, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub user_uuid: Uuid,
    pub user_role: UserRole,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    UserUuid,
    UserRole,
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
    Users,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::UserUuid => ColumnType::Uuid.def(),
            Self::UserRole => UserRole::db_type(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserUuid)
                .to(super::users::Column::UserUuid)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
