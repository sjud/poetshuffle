use web_sys::Performance;
use crate::services::network::GraphQlResp;
use super::*;
use crate::types::edit_set_context::{EditableSet, EditSetContext, EditSetActions};
use crate::types::edit_poem_list_context::{EditPoemListData,EditPoemListContext};
use poem_list::EditPoemList;

#[function_component(EditPendingSet)]
pub fn edit_pending_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    if use_is_first_mount() {
        let auth = auth_ctx.clone();
        let edit_set_ctx = edit_set_ctx.clone();
        let msg_ctx = msg_ctx.clone();
        let user_uuid = auth.user_uuid.unwrap();
        use_async::<_, (), String>(async move {
            match auth.pending_set_by_user__uuid(user_uuid).await? {
                GraphQlResp::Data(data) => {
                    if let Some(set) = data.pending_set_by_user {
                        edit_set_ctx.dispatch(
                            EditSetActions::EditableSet(
                                Some(EditableSet {
                                    set_uuid: Uuid::from_str(&set.set_uuid).unwrap(),
                                    link: set.link.clone(),
                                    title: set.title.clone(),
                                })));
                        } else {}
                },
                GraphQlResp::Err(errors) =>
                    msg_ctx.dispatch(errors.into_msg_action()),
            }
            Ok(())}
        ).run();
    }
    return html!{<EditPendingSetDecider/>};
}
#[function_component(EditPendingSetDecider)]
pub fn edit_pending_set_decider() -> Html {
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    if edit_set_ctx.editable_set.is_some() {
        let edit_poem_list_ctx = use_reducer(||EditPoemListData::default());
        return html! {
        <ContextProvider<EditPoemListContext> context={edit_poem_list_ctx}>
            <div>
            <h2>{"Edit Pending Set"}</h2>
            <UpdateSetTitle/>
            <br/>
            <UpdateSetLink/>
            <br/>
            <DeleteSet/>
            <br/>
            <EditPoemList/>
            </div>
        </ContextProvider<EditPoemListContext>>
    }; } else {
        return html! { <CreateSet/>};
    }
}

// TODO The following two components are basically the same, what would an abstracted UpdateSet component look like?
#[function_component(UpdateSetLink)]
pub fn update_set_link() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let editable_set = edit_set_context.editable_set.clone().unwrap();
    let (set_uuid, title, link) = editable_set.deconstruct();
    let link_ref = use_node_ref();
    let update_link = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_set_context = edit_set_context.clone();
        let link_ref = link_ref.clone();
        use_async::<_, (), String>(async move {
            let link = link_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_set(
                set_uuid,
                None,
                Some(link.clone()),
                None,
                None,).await? {
                GraphQlResp::Data(data) => {
                    edit_set_context.dispatch(EditSetActions::UpdateLink(link));
                    msg_context.dispatch(
                        new_green_msg_with_std_duration("Updated".to_string()));
                },
                GraphQlResp::Err(errors) => msg_context
                    .dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let update_link = Callback::from(move |_| {
        update_link.run();
    });
    return html! {
            <div>
            <h3>{"Link:"}</h3>
            <h4>{link}</h4>
            <input ref={link_ref.clone()}/>
            <button onclick={update_link.clone()}>{"Update Link"}</button>
            </div>
        };
}
#[function_component(DeleteSet)]
pub fn delete_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let editable_set = edit_set_context.editable_set.clone().unwrap();
    let (set_uuid,title, collection_link) = editable_set.deconstruct();
    let check_ref = use_node_ref();
    let delete = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_set_ctx = edit_set_context.clone();
        let check_ref = check_ref.clone();
        use_async::<_, (), String>(async move {
                match auth.update_set(
                    set_uuid,
                    None,
                    None,
                    Some(true),
                    None, ).await? {
                    GraphQlResp::Data(data) => {
                        edit_set_ctx.dispatch(EditSetActions::EditableSet(None));
                        msg_context.dispatch(new_green_msg_with_std_duration(data.update_set));
                    }
                    GraphQlResp::Err(errors) =>
                        msg_context.dispatch(errors.into_msg_action())
                }
            Ok(())
        })
    };
    let delete = Callback::from(move |_| {
        delete.run();
    });
    html! {
            <div>
            <button onclick={delete}>{"Delete Set"}</button>
            </div>
    }
}
#[function_component(UpdateSetTitle)]
pub fn update_set_title() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let editable_set = edit_set_context.editable_set.clone().unwrap();
    let (set_uuid,title, collection_link) = editable_set.deconstruct();
    let title_ref = use_node_ref();
    let update_title = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_set_ctx = edit_set_context.clone();
        let title_ref = title_ref.clone();
        use_async::<_, (), String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_set(
                set_uuid,
                Some(title.clone()),
                None,
                None,
                None,).await? {
                GraphQlResp::Data(data) => {
                    edit_set_ctx.dispatch(EditSetActions::UpdateTitle(title));
                    msg_context.dispatch(new_green_msg_with_std_duration(data.update_set));
                }
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let update_title = Callback::from(move |_| {
        update_title.run();
    });
    return html! {
            <div>
            <h3>{"Title:"}</h3>
            <h4>{title}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            </div>
        };
}


#[function_component(CreateSet)]
pub fn create_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let create_set = {
        let token = auth_ctx.token.clone();
        let msg_context = msg_context.clone();
        let edit_set_context = edit_set_context.clone();
        use_async::<_, (), String>(async move {
            let resp = post_graphql::<CreatePendingSetMutation>(
                create_pending_set_mutation::Variables {},
                token,
            )
            .await
            .map_err(|err| format!("{:?}", err))?;
            if let Some(ref data) = resp.data {
                edit_set_context.dispatch(EditSetActions::EditableSet(
                    Some(EditableSet {
                        set_uuid: Uuid::from_str(&data.create_pending_set.set_uuid).unwrap(),
                        link: data.create_pending_set.link.clone(),
                        title: data.create_pending_set.title.clone(),
                    })));
                msg_context.dispatch(
                    new_green_msg_with_std_duration("Set Created".to_string()));
            } else if resp.errors.is_some() {
                msg_context.dispatch(new_red_msg_with_std_duration(
                    map_graphql_errors_to_string(
                    &resp.errors,
                )));
                tracing::error!("{:?}", resp.errors);
            }
            Ok(())
        })
    };

    let create_set = Callback::from(move |_| {
        create_set.run();
    });
    html! {
        <button onclick={create_set.clone()}>{"Create New Set"}</button>
    }
}
