use crate::graphql::resolvers::sets::find_set_by_uuid;
use entity::poems::{self,ActiveModel as ActivePoem};
use entity::sea_orm_active_enums::SetStatus;
use super::*;

#[derive(Debug, sea_orm::FromQueryResult)]
struct PoemUuidResult {
    uuid:Uuid,
}

#[tracing::instrument(err)]
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
    use crate::graphql::test_util::{assert_no_graphql_errors_or_print_them, key_conn_email};
    use sea_orm::QueryTrait;

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
        assert_no_graphql_errors_or_print_them(result.errors);

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
        assert_no_graphql_errors_or_print_them(result.errors);
    }

}