//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "orders"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub order_uuid: Uuid,
    pub creation_ts: DateTimeWithTimeZone,
    pub purchasee_uuid: Uuid,
    pub set_uuid: Uuid,
    pub sent_to_address: String,
    pub gift: bool,
    pub email_delivery_confirmation: bool,
    pub purchase_price: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    OrderUuid,
    CreationTs,
    PurchaseeUuid,
    SetUuid,
    SentToAddress,
    Gift,
    EmailDeliveryConfirmation,
    PurchasePrice,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    OrderUuid,
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
            Self::OrderUuid => ColumnType::Uuid.def(),
            Self::CreationTs => ColumnType::TimestampWithTimeZone.def(),
            Self::PurchaseeUuid => ColumnType::Uuid.def(),
            Self::SetUuid => ColumnType::Uuid.def(),
            Self::SentToAddress => ColumnType::String(None).def(),
            Self::Gift => ColumnType::Boolean.def(),
            Self::EmailDeliveryConfirmation => ColumnType::Boolean.def(),
            Self::PurchasePrice => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::PurchaseeUuid)
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