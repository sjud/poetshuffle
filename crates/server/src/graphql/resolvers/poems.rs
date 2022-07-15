use super::*;
use entity::poems::{self, ActiveModel as ActivePoem};
use entity::sea_orm_active_enums::{SetStatus,UserRole};
use sea_orm::{ActiveValue, TransactionTrait};
use entity::prelude::Poems;
use crate::graphql::guards::*;

#[derive(Debug, sea_orm::FromQueryResult)]
struct PoemUuidResult {
    uuid: Uuid,
}
pub async fn find_poem_uuids_by_set_uuid(
    db: &DatabaseConnection,
    set_uuid: Uuid,
) -> Result<Vec<uuid::Uuid>> {
    Ok(entity::poems::Entity::find()
        .select_only()
        .column(poems::Column::PoemUuid)
        .column_as(poems::Column::PoemUuid, "uuid")
        .filter(poems::Column::SetUuid.eq(set_uuid))
        .into_model::<PoemUuidResult>()
        .all(db)
        .await
        .map_err(|err| Error::new(format!("Sea-orm: {:?}", err)))?
        .into_iter()
        .map(|poem| uuid::Uuid::from_u128(poem.uuid.as_u128()))
        .collect::<Vec<uuid::Uuid>>())
}

pub async fn add_poem(
    db: &DatabaseConnection,
    user_uuid: Uuid,
    set_uuid: Uuid,
) -> Result<poems::Model> {

    let poem_uuid = Uuid::new_v4();
    let idx = Poems::find()
        .filter(poems::Column::SetUuid.eq(set_uuid))
        .all(db)
        .await?
        .len();
    ActivePoem {
        poem_uuid: Set(poem_uuid),
        originator_uuid: Set(user_uuid),
        set_uuid: Set(set_uuid),
        idx: Set(idx as i32),
        title: Set("".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    if let Some(poem) = Poems::find_by_id(poem_uuid).one(db).await?{
        Ok(poem)
    } else {
        Err(Error::new("This is a weird error:\
         couldn't find poem after inserting into db..."))
    }
}


pub async fn find_poem(db: &DatabaseConnection, poem_uuid: Uuid) -> Result<Option<poems::Model>> {
    entity::poems::Entity::find_by_id(poem_uuid)
        .one(db)
        .await
        .map_err(|err| Error::new(format!("Sea-orm: {:?}", err)))
}
pub async fn find_poem_given_idx_set_uuid(db: &DatabaseConnection, set_uuid: Uuid, idx: i32) -> Result<Option<poems::Model>> {
        poems::Entity::find()
            .filter(poems::Column::SetUuid.eq(set_uuid))
            .filter(poems::Column::Idx.eq(idx))
            .one(db)
            .await
            .map_err(|err| Error::new(format!("Sea-orm: {:?}", err)))
}
pub fn build_edit_poem_value<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    poem: &poems::Model,
) -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        if auth.can_edit_poem(&poem) {
            return Ok(ActiveValue::set(value));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}

pub async fn find_poems_of_greater_idx(
    db:&DatabaseConnection,
    set_uuid:Uuid,
    idx:i32) -> Result<Vec<poems::Model>> {
    Poems::find()
        .filter(poems::Column::SetUuid.eq(set_uuid))
        .filter(poems::Column::Idx.gt(idx))
        .all(db)
        .await
        .map_err(|err| Error::new(format!("Sea-orm: {:?}", err)))
}

pub fn build_edit_poem_value_option<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    poem: &poems::Model,
) -> Result<ActiveValue<Option<V>>> {
    if let Some(value) = v {
        if auth.can_edit_poem(&poem) {
            return Ok(ActiveValue::set(Some(value)));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}

pub fn build_approve_value<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
) -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        if auth.can_approve() {
            return Ok(ActiveValue::set(value));
        } else {
            Err(Error::new("Unauthorized approval."))?;
        }
    }
    Ok(ActiveValue::not_set())
}

pub fn build_history_value_option<V: Into<sea_orm::Value> + migration::Nullable>(
    v: Option<V>,
) -> Result<ActiveValue<Option<V>>> {
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

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(set_uuid,OriginationCategory::Set))")]
    async fn add_poem(&self, ctx: &Context<'_>, set_uuid: Uuid) -> Result<poems::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_uuid = ctx.data::<Auth>()?
            .0
            .as_ref()
            .ok_or(Error::new("No permission in authorization."))?
            .user_uuid;
        let poem_uuid = Uuid::new_v4();
        let idx = Poems::find()
            .filter(poems::Column::SetUuid.eq(set_uuid))
            .all(db)
            .await?
            .len();
        ActivePoem {
            poem_uuid: Set(poem_uuid),
            originator_uuid: Set(user_uuid),
            set_uuid: Set(set_uuid),
            idx: Set(idx as i32),
            title: Set("".to_string()),
            ..Default::default()
        }
            .insert(db)
            .await?;
        if let Some(poem) = Poems::find_by_id(poem_uuid).one(db).await?{
            Ok(poem)
        } else {
            Err(Error::new("This is a weird error:\
         couldn't find poem after inserting into db..."))
        }
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(set_uuid,OriginationCategory::Set))")]
    pub async fn update_poem_idx(
        &self,
        ctx: &Context<'_>,
        set_uuid:Uuid,
        poem_a_idx:i32,
        poem_b_idx:i32) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Ok(Some(poem_a)) = find_poem_given_idx_set_uuid(db,set_uuid,poem_a_idx).await {
            if let Ok(Some(poem_b)) = find_poem_given_idx_set_uuid(db, set_uuid, poem_b_idx).await {
                let txn = db.begin().await?;
                ActivePoem{
                    poem_uuid:Set(poem_a.poem_uuid),
                    idx:Set(poem_b_idx),
                    ..poem_a.into()
                }.save(&txn).await?;
                ActivePoem{
                    poem_uuid:Set(poem_b.poem_uuid),
                    idx:Set(poem_a_idx),
                    ..poem_b.into()
                }.save(&txn).await?;
                txn.commit().await?;
                Ok("Idx Swap".into())
            } else {
                Err(Error::new("Poem B not found."))
            }
        } else {
            Err(Error::new("Poem A not found."))
        }
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Moderator)\
    .and(IsSetEditor::new(set_uuid))")]
    async fn set_approve_poem(
        &self,
        ctx: &Context<'_>,
        poem_uuid:Uuid,
        set_uuid:Uuid,
        approve: bool,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        ActivePoem {
            poem_uuid:Set(poem_uuid),
            is_approved: Set(approve),
            ..Default::default()
        }
            .update(db)
            .await?;
        Ok("Poem Approved".into())
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(poem_uuid,OriginationCategory::Poem))")]
    async fn delete_poem(&self, ctx:&Context<'_>, poem_uuid:Uuid)
                           -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Some(poem) = entity::poems::Entity::find_by_id(poem_uuid)
            .one(db)
            .await? {
            let txn = db.begin().await?;
            if let Some(banter_uuid) = poem.banter_uuid{
                entity::banters::ActiveModel{
                    banter_uuid:Set(banter_uuid),
                    ..Default::default()
                }.delete(&txn).await?;
            }
            let poems =
                find_poems_of_greater_idx(db,poem.set_uuid,poem.idx).await?;
            for poem in poems.iter() {
                ActivePoem {
                    poem_uuid:Set(poem.poem_uuid),
                    idx:Set(poem.idx-1),
                    ..Default::default()
                }
                    .update(&txn)
                    .await?;
            }
            ActivePoem{
                poem_uuid:Set(poem_uuid),
                ..Default::default()
            }.delete(&txn).await?;
            txn.commit().await?;
            Ok(String::from("Poem Deleted."))
        } else {
            Err("Poem not found".into())
        }
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(poem_uuid,OriginationCategory::Poem))")]
    async fn update_poem(
        &self,
        ctx: &Context<'_>,
        poem_uuid: Uuid,
        banter_uuid: Option<Uuid>,
        title: Option<String>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Some(poem) = find_poem(db, poem_uuid).await? {
            ActivePoem {
                poem_uuid: Set(poem_uuid),
                banter_uuid:  update_nullable_value(banter_uuid),
                title: update_value(title),
                ..Default::default()
            }
            .update(db)
            .await?;
            Ok("Poem Updated".into())
        } else {
            Err(Error::new("Can't update poem. Poem not found."))
        }
    }
}
#[Object]
impl PoemQuery {
    async fn poem(&self, ctx: &Context<'_>, poem_uuid: Uuid) -> Result<Option<poems::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        Ok({
            if let Some(poem) = find_poem(db, poem_uuid).await? {
                if auth.can_read_poem(&poem) {
                    Some(poem)
                } else {
                    Err(Error::new("Unauthorized."))?
                }
            } else {
                None
            }
        })
    }

    async fn poem_uuids_by_set_uuid(
        &self,
        ctx: &Context<'_>,
        set_uuid: Uuid,
    ) -> Result<Vec<uuid::Uuid>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        let set: entity::sets::Model = entity::prelude::Sets::find_by_id(set_uuid)
            .one(db)
            .await?
            .ok_or("Set not found")?;
        if set.set_status == SetStatus::Pending {
            if auth.can_read_pending_set(&set) {
                Ok(find_poem_uuids_by_set_uuid(db, set_uuid).await?)
            } else {
                Err(Error::new("Unauthorized."))
            }
        } else {
            Ok(find_poem_uuids_by_set_uuid(db, set_uuid).await?)
        }
    }
}
/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::resolvers::login::create_login_with_password;
    use crate::graphql::resolvers::sets::create_pending_set;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::{key_conn_email, no_graphql_errors_or_print_them};
    use entity::sea_orm_active_enums::UserRole;
    use sea_orm::DbBackend;
    use sea_orm::QueryTrait;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_poem_update() {
        let (key, conn, email) = key_conn_email().await;
        let user_uuid =
            create_login_with_password(&conn, "test_poem_update@test.com".into(), "1234".into())
                .await
                .unwrap();
        let schema = new_schema(conn.clone(), key.clone(), email);
        let set_uuid = create_pending_set(&conn, user_uuid).await.unwrap();
        let poem_uuid = add_poem(&conn, user_uuid, set_uuid, 0).await.unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",idx:1)
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
        assert_eq!(result.data.to_string(), "{updatePoem: \"\"}".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",idx:2)
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid: Uuid::new_v4(),
                    user_role: UserRole::Poet,
                }))),
            )
            .await;

        assert_eq!(result.errors[0].message, "Unauthorized update.".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\")
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        assert_eq!(
            result.errors[0].message,
            "Query Error: error returned from database: syntax error at or near \"WHERE\""
                .to_string()
        );
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",idx:3)
                }}",
                    poem_uuid
                ))
                .data(Auth(None)),
            )
            .await;
        assert_eq!(result.errors[0].message, "Unauthorized update.".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",idx:4)
                }}",
                    Uuid::new_v4()
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        assert_eq!(
            result.errors[0].message,
            "Can't update poem. Poem not found.".to_string()
        );
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",title:\"title\",idx:4,delete:false)
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",approve:true)
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        assert_eq!(
            result.errors[0].message,
            "Unauthorized approval.".to_string()
        );
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updatePoem(poemUuid:\"{}\",approve:true)
                }}",
                    poem_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Moderator,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_poem_uuids_by_set_uuid() {
        let (key, conn, email) = key_conn_email().await;

        let user_uuid = create_login_with_password(
            &conn,
            "test_poem_uuids_by_set_uuid@test.com".into(),
            "1234".into(),
        )
        .await
        .unwrap();
        let schema = new_schema(conn.clone(), key.clone(), email);
        let set_uuid = create_pending_set(&conn, user_uuid).await.unwrap();
        let poem_uuid = add_poem(&conn, user_uuid, set_uuid, 0).await.unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "query {{
                poemUuidsBySetUuid(setUuid:\"{}\")
                }}",
                    set_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
    }
    #[tokio::test]
    #[traced_test]
    async fn test_add_poem() {
        let (key, conn, email) = key_conn_email().await;

        let user_uuid =
            create_login_with_password(&conn, "test_add_poem@test.com".into(), "1234".into())
                .await
                .unwrap();
        let schema = new_schema(conn.clone(), key.clone(), email);
        let set_uuid = create_pending_set(&conn, user_uuid).await.unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                addPoem(setUuid:\"{}\",idx:0)
                }}",
                    set_uuid
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();
    }
}
*/