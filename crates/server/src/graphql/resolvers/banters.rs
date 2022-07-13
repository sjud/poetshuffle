use super::*;
use entity::banters::{self, ActiveModel as ActiveBanter};
use entity::poems::{self, ActiveModel as ActivePoem};

use entity::sea_orm_active_enums::SetStatus;
use sea_orm::{ActiveValue, TransactionTrait};
use entity::prelude::Banters;
use entity::prelude::Poems;
use crate::graphql::resolvers::poems::build_approve_value;


pub fn build_banter_edit_option_value<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    banter: &banters::Model,
) -> Result<ActiveValue<Option<V>>> {
    if let Some(value) = v {
        auth.can_edit_banter_v2(banter)?;
        Ok(ActiveValue::set(Some(value)))
    } else {
        Ok(ActiveValue::not_set())

    }
}
pub fn build_edit_banter_value<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    banter: &banters::Model,
) -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        auth.can_edit_banter_v2(banter)?;
        Ok(ActiveValue::set(value))
    } else {
        Ok(ActiveValue::not_set())
    }
}
#[derive(Default)]
pub struct BanterMutation;
#[Object]
impl BanterMutation {
    async fn set_approve_banter(
        &self,
        ctx: &Context<'_>,
        banter_uuid:Uuid,
        approve:bool,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx.data::<Auth>()?;
        if let Some(banter) = Banters::find_by_id(banter_uuid).one(db).await? {
            auth.can_approve_v2()?;
            ActiveBanter {
                banter_uuid:Set(banter_uuid),
                approved: Set(approve),
                ..Default::default()
            }
                .update(db)
                .await?;
            Ok("Banter Updated".into())
        } else {
            Err(Error::new("Can't update banter. Banter not found."))
        }
    }
    #[tracing::instrument(skip_all,err(Debug))]
    async fn add_banter(&self, ctx: &Context<'_>, poem_uuid: Uuid)
        -> Result<banters::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx.data::<Auth>()?;
        if let Some(poem) = Poems::find_by_id(poem_uuid).one(db).await?{
            let _ = auth.can_edit_poem_v2(&poem)?;
            let banter_uuid = Uuid::new_v4();
            let user_uuid = auth
                    .0
                    .as_ref()
                    .ok_or(
                        Error::new("Impossible authorization error???")
                    )?.user_uuid;
            ActiveBanter {
                banter_uuid: Set(banter_uuid),
                originator_uuid: Set(user_uuid),
                approved:Set(false),
                ..Default::default()
            }
                .insert(db)
                .await?;
            ActivePoem{
                    poem_uuid:Set(poem_uuid),
                    banter_uuid:Set(Some(banter_uuid)),
                ..Default::default()
            }.update(db)
                    .await?;
            if let Some(banter) = Banters::find_by_id(banter_uuid)
                .one(db).await?{
                    Ok(banter)
            } else {
                    Err(Error::new("This is a weird error:\
         couldn't find banter after inserting into db..."))
                }
        } else {
            Err(Error::new("No poem given poem_uuid."))
        }
    }
    async fn delete_banter(&self, ctx:&Context<'_>, poem_uuid:Uuid,banter_uuid:Uuid)
    -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let auth = ctx.data::<Auth>()?;
        if let Some(poem) = Poems::find_by_id(poem_uuid)
            .one(db)
            .await? {
            auth.can_edit_poem_v2(&poem)?;
            ActivePoem{
                poem_uuid:Set(poem_uuid),
                banter_uuid:Set(None),
                ..Default::default()
            }.update(db).await?;
            Banters::find_by_id(banter_uuid)
                .one(db)
                .await?
                .map(|banter|auth.can_edit_banter_v2(&banter))
                .ok_or(Error::new("Can't find banter."))?;
            ActiveBanter{
                banter_uuid:Set(banter_uuid),
                ..Default::default()
            }.delete(db)
                .await?;
            Ok(String::from("Banter Entry Deleted."))
        } else {
            Err("No associated poem found".into())
        }
    }
}