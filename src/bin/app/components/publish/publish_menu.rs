use super::*;
use super::edit_pending_set::*;
#[function_component(PublishMenu)]
pub fn publish_menu() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let editable_set : UseStateHandle<Option<EditableSet>> = use_state(||None);
    let poem_uuids : UseStateHandle<Option<Vec<Uuid>>> = use_state(||None);
    let new_edit_flag = use_state(||false);

    if use_is_first_mount() {
        new_edit_flag.set(true);
    }
    // If a user uuid is available find the pending set of the user.
    if let Some(user_uuid) = auth_ctx.user_uuid {
        let req = {
            let token = auth_ctx.token.clone();
            let pending_set = editable_set.clone();
            let msg_context = msg_context.clone();
            let new_edit_flag = new_edit_flag.clone();
            let poem_uuids = poem_uuids.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<PendingSetByUserQuery>(
                    pending_set_by_user_query::Variables {
                        user_uuid: user_uuid.to_string(),
                    }, token.clone())
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    if let Some(set) = &data.pending_set_by_user {
                        pending_set.set(Some(EditableSet {
                            set_uuid: Uuid::from_str(&set.set_uuid)
                                .unwrap(),
                            collection_link: set.collection_link.clone(),
                            collection_title: set.collection_title.clone(),
                        }));
                        // Make a new query with the found set uuid
                        let resp =
                                post_graphql::<PoemUuidsBySetUuidQuery>(
                                    poem_uuids_by_set_uuid_query::Variables{
                                        set_uuid:set.set_uuid.clone()
                                    },token)
                                    .await
                                    .map_err(|err| format!("{:?}", err))?;
                        if let Some(ref data) = resp.data {
                            poem_uuids.set(Some(
                                data
                                    .poem_uuids_by_set_uuid
                                    .iter()
                                    .map(|uuid|Uuid::from_str(&uuid).unwrap())
                                    .collect::<Vec<Uuid>>()));
                        } else if resp.errors.is_some() {
                            tracing::error!("{:?}", resp.errors);
                        }
                    };
                }
                // If we have no data then see if we have errors and print those to console.
                else if resp.errors.is_some() {
                    msg_context.dispatch(new_red_msg_with_std_duration(
                        map_graphql_errors_to_string(
                            &resp.errors
                        )
                    ));
                    tracing::error!("{:?}", resp.errors);
                }
                Ok(())
            })
        };
        if *new_edit_flag {
            req.run();
            new_edit_flag.set(false);
        }
    };

    let props = EditPendingSetProps{
        new_edit_flag,
        editable_set,
        poem_uuids,
    };

    html!{
        <div>
        <h2>{"Publish Menu"}</h2>
        <EditPendingSet ..props/>
        </div>
    }
}