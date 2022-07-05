use super::*;
use crate::graphql::resolvers::poems::{build_approve_value, build_history_value_option};
use entity::edit_set_history::ActiveModel as ActiveSetHistory;
use entity::prelude::Sets;
use entity::sea_orm_active_enums::SetStatus;
use entity::sets;
use entity::sets::ActiveModel as ActiveModelSet;
use sea_orm::ActiveValue;

pub fn build_edit_set_value<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    set: &sets::Model,
) -> Result<ActiveValue<V>> {
    if let Some(value) = v {
        if auth.can_edit_set(&set) {
            return Ok(ActiveValue::set(value));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}
pub fn build_edit_poem_value_option<V: Into<sea_orm::Value> + migration::Nullable>(
    auth: &Auth,
    v: Option<V>,
    set: &sets::Model,
) -> Result<ActiveValue<Option<V>>> {
    if let Some(value) = v {
        if auth.can_edit_set(&set) {
            return Ok(ActiveValue::set(Some(value)));
        } else {
            Err(Error::new("Unauthorized update."))?;
        }
    }
    Ok(ActiveValue::not_set())
}
pub async fn find_pending_set_by_user(
    db: &DatabaseConnection,
    user_uuid: Uuid,
) -> Result<Option<sets::Model>> {
    Sets::find()
        .filter(sets::Column::OriginatorUuid.eq(user_uuid))
        .filter(sets::Column::SetStatus.eq(SetStatus::Pending))
        .one(db)
        .await
        .map_err(|err| Error::new(format!("{:?}", err)))
}
pub async fn find_set_by_uuid(
    db: &DatabaseConnection,
    set_uuid: Uuid,
) -> Result<Option<sets::Model>> {
    Sets::find_by_id(set_uuid)
        .one(db)
        .await
        .map_err(|err| Error::new(format!("{:?}", err)))
}
pub async fn create_pending_set(db: &DatabaseConnection, user_uuid: Uuid) -> Result<sets::Model> {
    let set_uuid = Uuid::new_v4();
    entity::sets::ActiveModel {
        set_uuid: Set(set_uuid),
        title: Set(String::new()),
        originator_uuid: Set(user_uuid),
        set_status: Set(SetStatus::Pending),
        link: Set(String::new()),
        editor_uuid: Set(None),
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

#[derive(Default)]
pub struct SetMutation;
#[Object]
impl SetMutation {
    async fn update_set(
        &self,
        ctx: &Context<'_>,
        set_uuid: Uuid,
        title: Option<String>,
        link: Option<String>,
        delete: Option<bool>,
        approve: Option<bool>,
    ) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if let Ok(Some(set)) = find_set_by_uuid(db, set_uuid).await {
            ActiveModelSet {
                set_uuid: build_edit_set_value(auth, Some(set_uuid), &set)?,
                title: build_edit_set_value(auth, title.clone(), &set)?,
                link: build_edit_set_value(auth, link, &set)?,
                is_deleted: build_edit_set_value(auth, delete, &set)?,
                is_approved: build_approve_value(auth, approve)?,
                ..Default::default()
            }
            .update(db)
            .await?;
            ActiveSetHistory {
                history_uuid: Set(Uuid::new_v4()),
                user_uuid: Set(auth
                    .0
                    .as_ref()
                    //Checked above when inserting with poem_uuid... Test confirms.
                    .unwrap()
                    .user_uuid),
                set_uuid: Set(set_uuid),
                edit_title: build_history_value_option(title)?,
                is_deleted: build_history_value_option(delete)?,
                is_approved: build_history_value_option(approve)?,
                ..Default::default()
            }
            .insert(db)
            .await?;
            Ok("".into())
        } else {
            Err(Error::new("Can't update set. Set not found."))
        }
    }

    async fn create_pending_set(&self, ctx: &Context<'_>) -> Result<sets::Model> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let auth = ctx.data::<Auth>()?;
        if let Some(permission) = auth.0.clone() {
            if find_pending_set_by_user(db, permission.user_uuid)
                .await?
                .is_none()
            {
                let set = create_pending_set(db, permission.user_uuid).await?;
                Ok(set)
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
        Ok({
            if let Some(set) = find_pending_set_by_user(db, user_uuid).await? {
                if auth.can_read_pending_set(&set) {
                    Some(set)
                } else {
                    Err(Error::new("Unauthorized."))?
                }
            } else {
                None
            }
        })
    }
}
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
