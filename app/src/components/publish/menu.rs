use super::*;
use crate::components::publish::edit_pending_set::EditPendingSet;
use crate::services::network::GraphQlResp;
use crate::types::edit_set_context::{EditableSet, EditSetContext, EditSetDataActions};

#[function_component(PublishMenu)]
pub fn publish_menu() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if use_is_first_mount() {
        edit_set_context.dispatch(EditSetDataActions::NewEditFlag(true));
    }
    // If a user uuid is available find the pending set of the user.
    if let Some(user_uuid) = auth_ctx.user_uuid {
        let req = {
            let auth = auth_ctx.clone();
            let edit_set_ctx = edit_set_context.clone();
            let msg_context = msg_context.clone();
            use_async::<_, (), String>(async move {
                match auth.pending_set_by_user__uuid(user_uuid).await? {
                    GraphQlResp::Data(data) =>{
                        if let Some(set) = data.pending_set_by_user {
                            edit_set_ctx.dispatch(
                                EditSetDataActions::EditableSet(
                                    Some(EditableSet {
                                set_uuid: Uuid::from_str(&set.set_uuid).unwrap(),
                                link: set.link.clone(),
                                title: set.title.clone(),
                            })));
                            match auth.poem_uuids_by_set_uuid(
                                Uuid::from_str(&set.set_uuid).unwrap())
                                .await? {
                                GraphQlResp::Data(data) => {
                                    edit_set_ctx.dispatch(EditSetDataActions::PoemUuids(
                                        data.poem_uuids_by_set_uuid
                                            .iter()
                                            .map(|uuid| Uuid::from_str(&uuid).unwrap())
                                            .collect::<Vec<Uuid>>(),
                                    ));
                                },
                                GraphQlResp::Err(errors) => {
                                    msg_context.dispatch(errors.into_msg_action());
                                }
                            }
                        } else {
                            // there is so pending set so let user make one???
                        }
                    }
                    GraphQlResp::Err(errors) =>
                        msg_context.dispatch(errors.into_msg_action()),
                }
                Ok(())
            })
        };
        if edit_set_context.new_edit_flag {
            req.run();
            edit_set_context.dispatch(EditSetDataActions::NewEditFlag(false));
        }
    };

    return html! {
        <div>
        <h2>{"Publish Menu"}</h2>
        <EditPendingSet/>
        </div>
    };
}
