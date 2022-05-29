

use entity::prelude::Sets;
use entity::sea_orm_active_enums::SetStatus;
use entity::sets;
use crate::graphql::auth::{can_create_set, can_approve, can_change_status, can_edit_set};
use crate::types::Auth;
use super::*;

/*
    set_uuid UUID NOT NULL PRIMARY KEY,
    creation_ts TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    collection_title VARCHAR(100) NOT NULL,
    originator_uuid UUID NOT NULL REFERENCES users(user_uuid),
    set_status set_status NOT NULL,
    collection_link VARCHAR(250) NOT NULL,
    editor_uuid UUID REFERENCES users(user_uuid),
    approved BOOL NOT NULL
 */
#[derive(Default)]
pub struct SetsQuery;
#[Object]
impl SetsQuery{
    async fn set_uuids_by_status(&self, ctx:&Context<'_>,status:SetStatus) -> Result<Vec<sets::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let sets = Sets::find()
            .filter(sets::Column::SetStatus.eq(status))
            .all(db)
            .await?;
        Ok(sets)
    }
    async fn creation_ts(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<i64> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.creation_ts.timestamp())
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
    async fn collection_title(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.collection_title)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
    async fn originator_uuid(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<Uuid> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.originator_uuid)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
    async fn set_status(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<SetStatus> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.set_status)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
    async fn collection_link(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.collection_link)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
    async fn approved(&self, ctx:&Context<'_>,set_uuid:Uuid) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set.approved)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    }
}
#[derive(Default)]
pub struct SetsMutation;

#[Object]
impl SetsMutation{
    async fn create_set(&self,ctx:&Context<'_>,
        collection_title:String,
        originator_uuid:Uuid,
        collection_link:String, ) -> Result<Uuid> {
        if let Auth(Some(perm)) = ctx.data::<Auth>().unwrap() {
            let db = ctx.data::<DatabaseConnection>().unwrap();
            let uuid = Uuid::new_v4();
            if can_create_set(perm, originator_uuid)? {
                sets::ActiveModel {
                    set_uuid: Set(uuid),
                    collection_title: Set(collection_title),
                    originator_uuid: Set(originator_uuid),
                    set_status: Set(SetStatus::Pending),
                    collection_link: Set(collection_link),
                    approved: Set(false),
                    ..Default::default()
                }.insert(db).await?;
            }
            Ok(uuid)
        } else {
            Err(anyhow::Error::msg("No authorization provided"))
        }
    }
    async fn approved(&self, ctx:&Context<'_>,
    set_uuid:Uuid,
    approved:bool) -> Result<Uuid> {
        if let Auth(Some(perm)) = ctx.data::<Auth>().unwrap() {
            let db = ctx.data::<DatabaseConnection>().unwrap();
        if can_approve(perm)? {
            sets::ActiveModel{
                set_uuid:Set(set_uuid),
                approved:Set(approved),
                ..Default::default()
            }.update(db).await?;
        }
        Ok(set_uuid)
        } else {
            Err(anyhow::Error::msg("No authorization provided"))
        }
    }
    async fn set_status(&self, ctx:&Context<'_>,
    set_uuid:Uuid,
    set_status:SetStatus) -> Result<Uuid> {
        if let Auth(Some(perm)) = ctx.data::<Auth>().unwrap() {

            let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
                if can_change_status(perm,set.originator_uuid)? {
                    sets::ActiveModel{
                        set_uuid:Set(set_uuid),
                        set_status:Set(set_status),
                        ..Default::default()
                    }.update(db).await?;
                }
                Ok(set_uuid)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    } else {
    Err(anyhow::Error::msg("No authorization provided"))
    }
        
    }

    async fn collection_title(&self, ctx:&Context<'_>,
                              set_uuid:Uuid,
                              collection_title:String) -> Result<Uuid> {
        if let Auth(Some(perm)) = ctx.data::<Auth>().unwrap() {

            let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            if can_edit_set(perm,set.originator_uuid)? {
                    sets::ActiveModel{
                        set_uuid:Set(set_uuid),
                        collection_title:Set(collection_title),
                        ..Default::default()
                    }.update(db).await?;
                }
            Ok(set_uuid)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    } else {
Err(anyhow::Error::msg("No authorization provided"))
}
    }

    async fn collection_link(&self, ctx:&Context<'_>,
                             set_uuid:Uuid,
                             collection_link:String) -> Result<Uuid> {
        if let Auth(Some(perm)) = ctx.data::<Auth>().unwrap() {

            let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            if can_edit_set(perm,set.originator_uuid)? {
                sets::ActiveModel{
                    set_uuid:Set(set_uuid),
                    collection_link:Set(collection_link),
                    ..Default::default()
                }.update(db).await?;
            }
            Ok(set_uuid)
        } else {
            Err(anyhow::Error::msg(format!("Set not found, given uuid {}",set_uuid)))
        }
    } else {
Err(anyhow::Error::msg("No authorization provided"))
}
    }

}