use sea_orm::{ActiveValue, DbBackend, QueryTrait, Update};
use crate::graphql::resolvers::sets::find_set_by_uuid;
use entity::poems::{self,ActiveModel as ActivePoem};
use entity::edit_poem_history::{ActiveModel as ActivePoemHistory};
use entity::sea_orm_active_enums::SetStatus;
use super::*;

#[derive(Debug, sea_orm::FromQueryResult)]
struct PoemUuidResult {
    uuid:Uuid,
}

pub async fn find_poem_uuids_by_set_uuid(
    db:&DatabaseConnection,
    set_uuid:Uuid)
 -> Result<Vec<uuid::Uuid>> {
    Ok(
    entity::poems::Entity::find()
        .select_only()
        .column(poems::Column::PoemUuid)
        .column_as(poems::Column::PoemUuid, "uuid")
        .filter(poems::Column::SetUuid.eq(set_uuid))
        .into_model::<PoemUuidResult>()
        .all(db)
        .await
        .map_err(|err|{
            Error::new(format!("Sea-orm: {:?}",err))
        })?
        .into_iter()
        .map(|poem|uuid::Uuid::from_u128(poem.uuid.as_u128()))
        .collect::<Vec<uuid::Uuid>>()
        )
}
pub async fn add_poem(
    db:&DatabaseConnection,
    user_uuid:Uuid,
    set_uuid:Uuid,
    idx:i32,
) -> Result<Uuid> {
    let poem_uuid = Uuid::new_v4();
    ActivePoem{
        poem_uuid:Set(poem_uuid),
        originator_uuid:Set(user_uuid),
        set_uuid:Set(set_uuid),
        idx:Set(idx),
        title:Set("".to_string()),
        part_of_poetshuffle:Set(true),
        ..Default::default()
    }.insert(db).await?;
    Ok(poem_uuid)
}
pub async fn find_poem(
    db:&DatabaseConnection,
    poem_uuid:Uuid,
) -> Result<Option<poems::Model>> {
    entity::poems::Entity::find_by_id(poem_uuid)
        .one(db)
        .await
        .map_err(|err|{
            Error::new(format!("Sea-orm: {:?}",err))
        })
}
pub fn build_edit_poem_value<V:Into<sea_orm::Value>+ migration::Nullable>(auth:&Auth,v:Option<V>,poem:&poems::Model)
    -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        if auth.can_edit_poem(&poem) {
            return Ok(ActiveValue::set(value));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}
pub fn build_edit_poem_value_option<V:Into<sea_orm::Value>+ migration::Nullable>(auth:&Auth,v:Option<V>,poem:&poems::Model)
                                                                        -> Result<ActiveValue<Option<V>>> {
    if let Some(value) = v {
        if auth.can_edit_poem(&poem) {
            return Ok(ActiveValue::set(Some(value)));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}
pub fn build_approve_value<V:Into<sea_orm::Value>+ migration::Nullable>(auth:&Auth,v:Option<V>)
                                                                               -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        if auth.can_approve() {
            return Ok(ActiveValue::set(value));
        } else {
            Err(Error::new("Unauthorized approval."))?;
        }
    }
    Ok(ActiveValue::not_set())
}
pub fn build_history_value_option<V:Into<sea_orm::Value>+ migration::Nullable>(v:Option<V>)
                                                                                 -> Result<ActiveValue<Option<V>>> {
    if let Some(value) = v {
        return Ok(ActiveValue::set(Some(value)));
    }
    Ok(ActiveValue::not_set())
}
#[derive(Default)]
pub struct PoemMutation;
#[derive(Default)]
pub struct PoemQuery;
#[Object]
impl PoemMutation {
    async fn add_poem(
        &self,
        ctx:&Context<'_>,
        set_uuid:Uuid,
        idx:i32,
    ) -> Result<uuid::Uuid> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if auth.can_edit_set(
            &find_set_by_uuid(db,set_uuid)
                .await?.ok_or("Set not found")?
        ) {
            let user_uuid = auth.0.as_ref().ok_or(
                Error::new("Impossible authorization error."))?
                .user_uuid;
            let poem_uuid = add_poem(db,user_uuid,set_uuid,idx)
                .await?;
            Ok(uuid::Uuid::from_u128(poem_uuid.as_u128()))
        } else {
            Err(Error::new("Unauthorized."))
        }
    }
}
#[Object]
impl PoemQuery {
    async fn poem(
        &self,
        ctx:&Context<'_>,
        poem_uuid:Uuid
    ) -> Result<Option<poems::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        Ok({if let Some(poem) = find_poem(db,poem_uuid).await? {
            if auth.can_read_poem(&poem) {
                Some(poem)
            } else {
                Err(Error::new("Unauthorized."))?
            }
        } else { None }
        })
    }
    async fn update_poem(
        &self,
        ctx:&Context<'_>,
        poem_uuid:Uuid,
        banter_uuid:Option<Uuid>,
        title:Option<String>,
        idx:Option<i32>,
        is_deleted:Option<bool>,
        is_approved:Option<bool>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if let Some(poem) = find_poem(db,poem_uuid).await? {
            ActivePoem{
                poem_uuid:build_edit_poem_value(auth,Some(poem_uuid),&poem)?,
                    banter_uuid:build_edit_poem_value_option(auth,banter_uuid,&poem)?,
                    title:build_edit_poem_value(auth,title.clone(),&poem)?,
                    idx:build_edit_poem_value(auth,idx,&poem)?,
                    is_deleted:build_edit_poem_value(auth,is_deleted,&poem)?,
                    is_approved:build_approve_value(auth,is_approved)?,
                    ..Default::default()
                }.update(db).await?;
            ActivePoemHistory{
                history_uuid:Set(Uuid::new_v4()),
                user_uuid:Set(
                    auth.0
                        .as_ref()
                        //Checked above when inserting with poem_uuid... Test confirms.
                        .unwrap()
                        .user_uuid),
                poem_uuid:Set(poem_uuid),
                edit_title:build_history_value_option(title,)?,
                edit_idx:build_history_value_option(idx,)?,
                edit_is_deleted:build_history_value_option(is_deleted,)?,
                edit_is_approved:build_history_value_option(is_approved)?,
                edit_banter_uuid:build_history_value_option(banter_uuid)?,
                ..Default::default()
            }.insert(db).await?;
                Ok("".into())
        } else {
            Err(Error::new("Can't update poem. Poem not found."))
        }
    }
    async fn poem_uuids_by_set_uuid(
        &self,
        ctx:&Context<'_>,
        set_uuid:Uuid
    ) -> Result<Vec<uuid::Uuid>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        let set : entity::sets::Model =
            find_set_by_uuid(db,set_uuid)
                .await?
                .ok_or("Set not found")?;
        if set.set_status == SetStatus::Pending {
            if auth.can_read_pending_set(&set) {
                Ok(
                    find_poem_uuids_by_set_uuid(db,set_uuid).await?
                )
            } else {
                Err(Error::new("Unauthorized."))
            }
        } else {
            Ok(
                find_poem_uuids_by_set_uuid(db,set_uuid).await?
            )
        }
    }
}

#[cfg(test)]
mod test {
    use sea_orm::DbBackend;
    use super::*;
    use tracing_test::traced_test;
    use entity::sea_orm_active_enums::UserRole;
    use crate::graphql::resolvers::login::create_login_with_password;
    use crate::graphql::resolvers::sets::create_pending_set;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::{no_graphql_errors_or_print_them, key_conn_email};
    use sea_orm::QueryTrait;


    #[tokio::test]
    #[traced_test]
    async fn test_poem_update() {
        let (key,conn,email) = key_conn_email().await;
        let user_uuid = create_login_with_password(
            &conn,
            "test_poem_update@test.com".into(),
            "1234".into())
            .await
            .unwrap();
        let schema = new_schema(conn.clone(),key.clone(),email);
        let set_uuid = create_pending_set(&conn,user_uuid)
            .await
            .unwrap();
        let poem_uuid = add_poem(
            &conn,
            user_uuid,
            set_uuid,
            0)
            .await
            .unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",idx:1)
                }}",poem_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
        assert_eq!(result.data.to_string(), "{updatePoem: \"\"}".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",idx:2)
                }}",poem_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid:Uuid::new_v4(), user_role: UserRole::Poet })
            )))
            .await;

        assert_eq!(result.errors[0].message,"Unauthorized update.".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\")
                }}",poem_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        assert_eq!(result.errors[0].message,"Query Error: error returned from database: syntax error at or near \"WHERE\"".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",idx:3)
                }}",poem_uuid)
            ).data(Auth(
                None
            )))
            .await;
        assert_eq!(result.errors[0].message,"Unauthorized update.".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",idx:4)
                }}",Uuid::new_v4())
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        assert_eq!(result.errors[0].message,"Can't update poem. Poem not found.".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",title:\"title\",idx:4,isDeleted:false)
                }}",poem_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                updatePoem(poemUuid:\"{}\",isApproved:true)
                }}",poem_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        assert_eq!(result.errors[0].message,"Unauthorized approval.".to_string());


    }

    #[tokio::test]
    #[traced_test]
    async fn test_poem_uuids_by_set_uuid() {
        let (key,conn,email) = key_conn_email().await;

        let user_uuid = create_login_with_password(
            &conn,
            "test_poem_uuids_by_set_uuid@test.com".into(),
            "1234".into())
            .await
            .unwrap();
        let schema = new_schema(conn.clone(),key.clone(),email);
        let set_uuid = create_pending_set(&conn,user_uuid)
            .await
            .unwrap();
        let poem_uuid = add_poem(&conn,user_uuid,set_uuid,0).await.unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                poemUuidsBySetUuid(setUuid:\"{}\")
                }}",set_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();

    }
    #[tokio::test]
    #[traced_test]
    async fn test_add_poem() {
        let (key,conn,email) = key_conn_email().await;

        let user_uuid = create_login_with_password(
            &conn,
            "test_add_poem@test.com".into(),
            "1234".into())
            .await
            .unwrap();
        let schema = new_schema(conn.clone(),key.clone(),email);
        let set_uuid = create_pending_set(&conn,user_uuid)
            .await
            .unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                format!("mutation {{
                addPoem(setUuid:\"{}\",idx:0)
                }}",set_uuid)
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
    }

}