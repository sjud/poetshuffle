use super::*;
use entity::prelude::Sets;
use entity::sets::ActiveModel as ActiveModelSet;
use entity::sea_orm_active_enums::SetStatus;
use entity::sets;

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
pub async fn find_pending_set_by_user(
    db:&DatabaseConnection,
    user_uuid:Uuid
) -> Result<Option<sets::Model>> {
    Sets::find()
        .filter(sets::Column::OriginatorUuid.eq(user_uuid))
        .filter(sets::Column::SetStatus.eq(SetStatus::Pending))
        .one(db)
        .await
        .map_err(|err|Error::new(format!("{:?}",err)))
}
pub async fn find_set_by_uuid(
    db:&DatabaseConnection,
    set_uuid:Uuid
) -> Result<Option<sets::Model>> {
    Sets::find_by_id(set_uuid)
        .one(db)
        .await
        .map_err(|err|Error::new(format!("{:?}",err)))
}
pub async fn create_pending_set(
    db:&DatabaseConnection,
    user_uuid:Uuid
) -> Result<Uuid> {
    let set_uuid = Uuid::new_v4();
    entity::sets::ActiveModel{
        set_uuid: Set(set_uuid),
        collection_title: Set(String::new()),
        originator_uuid: Set(user_uuid),
        set_status: Set(SetStatus::Pending),
        collection_link: Set(String::new()),
        editor_uuid: Set(None),
        approved: Set(false),
        ..Default::default()
    }.insert(db).await?;
    Ok(set_uuid)
}

#[derive(Default)]
pub struct SetMutation;
#[Object]
impl SetMutation{
    async fn update_set(
        &self,
        ctx:&Context<'_>,
        set_uuid:Uuid,
        title:Option<String>,
        link:Option<String>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if let Ok(Some(set)) = find_set_by_uuid(db,set_uuid).await {
            if auth.can_edit_set(&set) {
                ActiveModelSet{
                    set_uuid:Set(set.set_uuid),
                    collection_title:{
                        if let Some(title) = title {
                            Set(title)
                        } else {
                            Default::default()
                        }
                    },
                    collection_link:{
                        if let Some(link) = link {
                            Set(link)
                        } else {
                            Default::default()
                        }
                    },
                    ..Default::default()
                }.update(db).await?;
                Ok("".into())
            } else {
                Err(Error::new("Unauthorized"))
            }
        } else {
            Err(Error::new("Set not found."))
        }
    }

    async fn create_pending_set(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Uuid> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if let Some(permission) = auth.0.clone() {
            if find_pending_set_by_user(db,permission.user_uuid)
                .await?
                .is_none() {
                create_pending_set(
                    db,
                    permission.user_uuid,
                ).await
            } else {
                Err(Error::new("You have an ongoing pending set."))
            }
        } else {
            Err(Error::new("Not logged in."))
        }
    }
}

#[derive(Default)]
pub struct SetsQuery;

#[Object]
impl SetsQuery {
    async fn pending_set_by_user(
        &self,
        ctx: &Context<'_>,
        user_uuid: Uuid,
    ) -> Result<Option<sets::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        Ok({if let Some(set) = find_pending_set_by_user(db,user_uuid).await? {
            if auth.can_read_pending_set(&set) {
                Some(set)
            } else {
                Err(Error::new("Unauthorized."))?
            }
        } else { None }
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;
    use entity::sea_orm_active_enums::UserRole;
    use crate::graphql::resolvers::login::create_login_with_password;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::key_conn_email;

    #[tokio::test]
    #[traced_test]
    async fn test_pending_set_by_user() {
        let (key,conn,email) = key_conn_email().await;
        let schema = new_schema(conn.clone(),key,email);
        let user_uuid = create_login_with_password(
            &conn,
            "test_pending_set_by_user@test.com".to_string(),
            "1234".to_string()
        ).await.unwrap();
        let set_uuid = Uuid::new_v4();
        entity::sets::ActiveModel{
            set_uuid: Set(set_uuid),
            collection_title: Set(String::from("Hello")),
            originator_uuid: Set(user_uuid),
            set_status: Set(SetStatus::Pending),
            collection_link: Set(String::new()),
            editor_uuid: Set(None),
            approved: Set(false),
            ..Default::default()
        }.insert(&conn).await.unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                format!("query {{
                pendingSetByUser(userUuid:\"{}\") {{
                    setUuid
                    collectionTitle
                    collectionLink
                    }}
                }}",user_uuid.to_string())
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        crate::graphql::test_util::assert_no_graphql_errors_or_print_them(
            result.errors
        );
        assert_eq!(result.data.to_string(),
                   format!("{{pendingSetByUser: \
                   {{setUuid: \"{}\",collectionTitle: \"Hello\",collectionLink: \"\"}}}}",set_uuid));
    }
    #[tokio::test]
    #[traced_test]
    async fn test_create_pending_set() {
        let (key,conn,email) = key_conn_email().await;
        let schema = new_schema(conn.clone(),key,email);
        let user_uuid = create_login_with_password(
            &conn,
            "test_create_pending_set_@test.com".to_string(),
            "1234".to_string()
        ).await.unwrap();
        let result = schema
            .execute(async_graphql::Request::from(
                "mutation{
                    createPendingSet
                }"
            ).data(Auth(
            Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
        )))
        .await;
        crate::graphql::test_util::assert_no_graphql_errors_or_print_them(
            result.errors
        );
        let result = schema
            .execute(async_graphql::Request::from(
                "mutation{
                    createPendingSet
                }"
            ).data(Auth(
                None
            ))).await;
        assert_eq!(result.errors[0].to_string(),"Not logged in.".to_string());
        let result = schema
            .execute(async_graphql::Request::from(
                "mutation{
                    createPendingSet
                }"
            ).data(Auth(
                Some(entity::permissions::Model{ user_uuid, user_role: UserRole::Poet })
            )))
            .await;
        assert_eq!(result.errors[0].to_string(),"You have an ongoing pending set.".to_string());

    }
}
