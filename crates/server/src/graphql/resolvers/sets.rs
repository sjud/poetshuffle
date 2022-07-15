use super::*;
use entity::prelude::Sets;
use entity::sea_orm_active_enums::{SetStatus,UserRole};
use entity::sets;
use entity::sets::ActiveModel as ActiveSet;
use sea_orm::ActiveValue;
use crate::graphql::guards::*;


#[derive(Default)]
pub struct SetMutation;
#[Object]
impl SetMutation {
    #[graphql(guard = "MinRoleGuard::new(UserRole::Moderator)\
    .and(IsSetEditor::new(set_uuid))")]
    async fn set_approve_set(
        &self,
        ctx:&Context<'_>,
        set_uuid:Uuid,
        approve:bool
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        ActiveSet {
            set_uuid:Set(set_uuid),
            is_approved: Set(approve),
            ..Default::default()
        }
            .update(db)
            .await?;
        Ok("Set Updated".into())
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(IsOriginator::new(set_uuid,OriginationCategory::Set))")]
    async fn update_set(
        &self,
        ctx: &Context<'_>,
        set_uuid: Uuid,
        title: Option<String>,
        link: Option<String>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        ActiveSet{
            set_uuid:Set(set_uuid),
            title: update_value(title),
            link: update_value(link),
            ..Default::default()
        }.update(db)
            .await?;
        Ok("Set Updated".into())
    }

    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(UniquePendingSet)")]
    async fn create_pending_set(&self, ctx: &Context<'_>) -> Result<sets::Model> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let user_uuid = ctx.data::<Auth>()?
            .0
            .as_ref()
            .ok_or("No permission in auth.")?
            .user_uuid;
        let set_uuid = Uuid::new_v4();
        entity::sets::ActiveModel {
            set_uuid: Set(set_uuid),
            title: Set(String::new()),
            originator_uuid: Set(user_uuid),
            set_status: Set(SetStatus::Pending),
            link: Set(String::new()),
            is_approved: Set(false),
            ..Default::default()
        }
            .insert(db)
            .await?;
        if let Some(set) = Sets::find_by_id(set_uuid)
            .one(db)
            .await? {
            Ok(set)
        } else {
            Err(Error::new("Can't find set that was just created"))
        }
    }
}

#[derive(Default)]
pub struct SetsQuery;

#[Object]
impl SetsQuery {
    #[graphql(guard = "MinRoleGuard::new(UserRole::Poet)\
    .and(AboutSelf::new(user_uuid))\
    .or(MinRoleGuard::new(UserRole::Moderator))")]
    async fn pending_set_by_user(
        &self,
        ctx: &Context<'_>,
        user_uuid: Uuid,
    ) -> Result<Option<sets::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(Sets::find()
            .filter(sets::Column::OriginatorUuid.eq(user_uuid))
            .filter(sets::Column::IsDeleted.eq(false))
            .filter(sets::Column::SetStatus.eq(SetStatus::Pending))
            .one(db)
            .await?)

    }
}
/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::resolvers::login::create_login_with_password;
    use crate::graphql::schema::new_schema;
    use crate::graphql::test_util::{key_conn_email, no_graphql_errors_or_print_them};
    use entity::sea_orm_active_enums::UserRole;
    use tracing_test::traced_test;
    #[tokio::test]
    #[traced_test]
    async fn test_set_update() {
        let (key, conn, email) = key_conn_email().await;
        let user_uuid =
            create_login_with_password(&conn, "test_set_update@test.com".into(), "1234".into())
                .await
                .unwrap();
        let schema = new_schema(conn.clone(), key.clone(), email);
        let set_uuid = create_pending_set(&conn, user_uuid).await.unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updateSet(setUuid:\"{}\",title:\"\")
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
        assert_eq!(result.data.to_string(), "{updateSet: \"\"}".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updateSet(setUuid:\"{}\",title:\"\")
                }}",
                    set_uuid
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
                updateSet(setUuid:\"{}\",title:\"\")
                }}",
                    set_uuid
                ))
                .data(Auth(None)),
            )
            .await;
        assert_eq!(result.errors[0].message, "Unauthorized update.".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updateSet(setUuid:\"{}\",title:\"\")
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
            "Can't update set. Set not found.".to_string()
        );
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updateSet(setUuid:\"{}\",title:\"title\",link:\"link\",delete:false)
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
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "mutation {{
                updateSet(setUuid:\"{}\",approve:true)
                }}",
                    set_uuid
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
                updateSet(setUuid:\"{}\",approve:true)
                }}",
                    set_uuid
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
    async fn test_pending_set_by_user() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key, email);
        let user_uuid = create_login_with_password(
            &conn,
            "test_pending_set_by_user@test.com".to_string(),
            "1234".to_string(),
        )
        .await
        .unwrap();
        let set_uuid = Uuid::new_v4();
        entity::sets::ActiveModel {
            set_uuid: Set(set_uuid),
            title: Set(String::from("Hello")),
            originator_uuid: Set(user_uuid),
            set_status: Set(SetStatus::Pending),
            link: Set(String::new()),
            editor_uuid: Set(None),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(format!(
                    "query {{
                pendingSetByUser(userUuid:\"{}\") {{
                    setUuid
                    title
                    link
                    }}
                }}",
                    user_uuid.to_string()
                ))
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();

        assert_eq!(
            result.data.to_string(),
            format!(
                "{{pendingSetByUser: \
                   {{setUuid: \"{}\",title: \"Hello\",link: \"\"}}}}",
                set_uuid
            )
        );
    }
    #[tokio::test]
    #[traced_test]
    async fn test_create_pending_set() {
        let (key, conn, email) = key_conn_email().await;
        let schema = new_schema(conn.clone(), key, email);
        let user_uuid = create_login_with_password(
            &conn,
            "test_create_pending_set_@test.com".to_string(),
            "1234".to_string(),
        )
        .await
        .unwrap();
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation{
                    createPendingSet
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        no_graphql_errors_or_print_them(result.errors).unwrap();

        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation{
                    createPendingSet
                }",
                )
                .data(Auth(None)),
            )
            .await;
        assert_eq!(result.errors[0].to_string(), "Not logged in.".to_string());
        let result = schema
            .execute(
                async_graphql::Request::from(
                    "mutation{
                    createPendingSet
                }",
                )
                .data(Auth(Some(entity::permissions::Model {
                    user_uuid,
                    user_role: UserRole::Poet,
                }))),
            )
            .await;
        assert_eq!(
            result.errors[0].to_string(),
            "You have an ongoing pending set.".to_string()
        );
    }
}
*/