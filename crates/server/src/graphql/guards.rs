use entity::sea_orm_active_enums::{SetStatus, UserRole};
use async_graphql::*;
use sea_orm::{ColumnTrait, DatabaseConnection};
use crate::types::auth::{Auth, OrdRoles};
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::Uuid;

pub struct IsApproved{
    item_uuid:Uuid,
    approved_cat:ApprovedCategory
}
pub enum ApprovedCategory{
    Set,
    Intro,
    Poem,
    Banter
}
impl IsApproved{
    pub fn new(item_uuid:Uuid,approved_cat:ApprovedCategory) -> Self {
        Self{ item_uuid, approved_cat}
    }
}
#[async_trait::async_trait]
impl Guard for IsApproved {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let db = ctx.data::<DatabaseConnection>()?;
        let is_approved = match self.approved_cat{
            ApprovedCategory::Set =>  entity::poems::Entity::find()
                .filter(entity::poems::Column::PoemUuid.eq(
                    self.item_uuid
                ))
                .filter(entity::banters::Column::Approved.eq(
                    true
                ))
                .one(db)
                .await?
                .is_some(),
            ApprovedCategory::Intro =>
                entity::intros::Entity::find()
                    .filter(entity::intros::Column::IntroUuid.eq(
                        self.item_uuid
                    ))
                    .filter(entity::intros::Column::Approved.eq(
                        true
                    ))
                    .one(db)
                    .await?
                    .is_some(),
            ApprovedCategory::Poem =>
                entity::poems::Entity::find()
                    .filter(entity::poems::Column::PoemUuid.eq(
                        self.item_uuid

                    ))
                    .filter(entity::banters::Column::Approved.eq(
                        true
                    ))
                    .one(db)
                    .await?
                    .is_some(),
            ApprovedCategory::Banter =>
                entity::banters::Entity::find()
                    .filter(entity::banters::Column::BanterUuid.eq(
                        self.item_uuid
                    ))
                    .filter(entity::banters::Column::Approved.eq(
                        true
                    ))
                    .one(db)
                    .await?
                    .is_some()
        };
        if is_approved {
            Ok(())
        } else {
            Err("A match between originator uuid, \
                item uuid, and user uuid does not exist.".into())
        }
    }
}

pub struct AboutSelf{
    user_uuid:Uuid,
}
impl AboutSelf{
    pub fn new(user_uuid:Uuid) -> Self { Self { user_uuid } }
}
#[async_trait::async_trait]
impl Guard for AboutSelf {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let user_uuid = ctx
            .data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("Permission not found"))?
            .user_uuid;
         if self.user_uuid == user_uuid {
             Ok(())
         } else {
             Err("User uuid given, doesn't match user uuid in authorization.".into())
         }
    }
}

/// MinRoleGuard's guard impl checks that the role in the authorization,
/// provided by the  JWT in the request Is equal to or greater than
/// the role inside of MinRoleGuard.
pub struct MinRoleGuard{
    user_role:UserRole,
}

impl MinRoleGuard {
    pub fn new(user_role: UserRole) -> Self {
        Self { user_role }
    }
}

#[async_trait::async_trait]
impl Guard for MinRoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let user_role = ctx
            .data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("Permission not found"))?
            .user_role;
        if OrdRoles(user_role) >= OrdRoles(self.user_role) {
            Ok(())
        } else {
            Err("Unauthorized".into())
        }
    }
}

pub struct IsSetEditor{
    set_uuid:Uuid,
}

impl IsSetEditor{
    pub fn new(set_uuid:Uuid) -> Self {
        Self { set_uuid }
    }
}

#[async_trait::async_trait]
impl Guard for IsSetEditor{
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let db = ctx
            .data::<DatabaseConnection>()?;
        let user_uuid = ctx.data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("Can't find permission."))?
            .user_uuid;
        entity::set_editors::Entity::find_by_id((self.set_uuid,user_uuid))
            .one(db)
            .await?
            .ok_or(Error::new("User is not an editor of set."))?;
        Ok(())
    }
}
/// The set of things that have an originating user.
pub enum OriginationCategory{
    Set,
    Intro,
    Poem,
    Banter
}

pub struct IsOriginator{
    item_uuid:Uuid,
    orig_cat:OriginationCategory,
}
impl IsOriginator{
    pub fn new(item_uuid:Uuid,orig_cat:OriginationCategory) -> Self {
        Self{
            item_uuid,
            orig_cat
        }
    }
}
#[async_trait::async_trait]
impl Guard for IsOriginator {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_uuid = ctx
            .data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("Can't find permission."))?
            .user_uuid;
        let is_originator = match self.orig_cat{
            OriginationCategory::Set =>  entity::poems::Entity::find()
                .filter(entity::poems::Column::PoemUuid.eq(
                    self.item_uuid
                ))
                .filter(entity::banters::Column::OriginatorUuid.eq(
                    user_uuid
                ))
                .one(db)
                .await?
                .is_some(),
            OriginationCategory::Intro =>
                entity::intros::Entity::find()
                    .filter(entity::intros::Column::IntroUuid.eq(
                        self.item_uuid
                    ))
                    .filter(entity::intros::Column::OriginatorUuid.eq(
                        user_uuid
                    ))
                    .one(db)
                    .await?
                    .is_some(),
            OriginationCategory::Poem =>
                entity::poems::Entity::find()
                    .filter(entity::poems::Column::PoemUuid.eq(
                        self.item_uuid

                    ))
                    .filter(entity::banters::Column::OriginatorUuid.eq(
                        user_uuid
                    ))
                    .one(db)
                    .await?
                    .is_some(),
            OriginationCategory::Banter =>
                entity::banters::Entity::find()
                    .filter(entity::banters::Column::BanterUuid.eq(
                        self.item_uuid
                    ))
                    .filter(entity::banters::Column::OriginatorUuid.eq(
                        user_uuid
                    ))
                    .one(db)
                    .await?
                    .is_some()
        };
        if is_originator {
            Ok(())
        } else {
            Err("A match between originator uuid, \
                item uuid, and user uuid does not exist.".into())
        }
    }
}

pub struct UniquePendingSet;
#[async_trait::async_trait]
impl Guard for UniquePendingSet {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_uuid = ctx
            .data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("Can't find permission."))?
            .user_uuid;
        let unique = entity::prelude::Sets::find()
            .filter(entity::sets::Column::OriginatorUuid.eq(user_uuid))
            .filter(entity::sets::Column::SetStatus.eq(SetStatus::Pending))
            .one(db)
            .await?
            .is_none();
        if unique{
            Ok(())
        } else {
            Err("You can't create a new pending set.\
             A pending set already exists for you.".into())
        }
    }
}
