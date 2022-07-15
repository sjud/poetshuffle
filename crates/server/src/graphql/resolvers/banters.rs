use super::*;
use entity::banters::{self, ActiveModel as ActiveBanter};
use entity::poems::{self, ActiveModel as ActivePoem};

use entity::sea_orm_active_enums::SetStatus;
use sea_orm::{ActiveValue, TransactionTrait};
use entity::prelude::Banters;
use entity::prelude::Poems;
use crate::graphql::resolvers::poems::build_approve_value;
use entity::sea_orm_active_enums::UserRole;
use super::super::guards::{
    MinRoleGuard,
    IsSetEditor,
    OriginationCategory,
    IsOriginator};

#[derive(Default)]
pub struct BanterMutation;

#[Object]
impl BanterMutation {
    #[graphql(guard = "MinRoleGuard::new(UserRole::Moderator)\
    .and(IsSetEditor::new(set_uuid))")]
    async fn set_approve_banter(
        &self,
        ctx: &Context<'_>,
        banter_uuid:Uuid,
        set_uuid:Uuid,
        approve:bool,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        ActiveBanter {
            banter_uuid:Set(banter_uuid),
            approved: Set(approve),
            ..Default::default()
        }
            .update(db)
            .await?;
        Ok("Banter Updated".into())
    }
    #[tracing::instrument(skip_all,err(Debug))]
    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(poem_uuid,OriginationCategory::Poem))")]
    async fn add_banter(&self, ctx: &Context<'_>, poem_uuid: Uuid)
        -> Result<banters::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_uuid = ctx.data::<Auth>()?
            .0
            .as_ref()
            .ok_or("No permission in auth.")?
            .user_uuid;
        let banter_uuid = Uuid::new_v4();
        let txn = db.begin().await?;
        ActiveBanter {
                banter_uuid: Set(banter_uuid),
                originator_uuid: Set(user_uuid),
                approved:Set(false),
                ..Default::default()
        }
                .insert(&txn)
                .await?;
        ActivePoem{
                    poem_uuid:Set(poem_uuid),
                    banter_uuid:Set(Some(banter_uuid)),
                ..Default::default()
            }.update(&txn)
                    .await?;
        txn.commit().await?;
        if let Some(banter) = entity::prelude::Banters::find_by_id(banter_uuid)
            .one(db)
            .await?{
            Ok(banter)
        } else {
            Err("Can't find banter that was just inserted...".into())
        }
    }
    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(banter_uuid,OriginationCategory::Banter))")]
    async fn delete_banter(&self, ctx:&Context<'_>, poem_uuid:Uuid,banter_uuid:Uuid)
    -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx.data::<Auth>()?;
        let txn = db.begin().await?;

        ActivePoem{
                poem_uuid:Set(poem_uuid),
                banter_uuid:Set(None),
                ..Default::default()
        }.update(&txn).await?;
        ActiveBanter{
                banter_uuid:Set(banter_uuid),
                ..Default::default()
        }.delete(&txn)
                .await?;
        txn.commit().await?;

        Ok(String::from("Banter Entry Deleted."))
        }
}
