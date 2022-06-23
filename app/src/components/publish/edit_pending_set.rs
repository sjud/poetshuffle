use crate::services::network::GraphQlResp;
use super::*;
use crate::types::edit_set_context::{EditSetContext, EditSetDataActions};

#[function_component(EditPendingSet)]
pub fn edit_pending_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if let Some(editable_set) = (edit_set_context.editable_set).clone() {
        // We use the title we got pass from props.
        return html! {
            <div>
        <h2>{"Edit Pending Set"}</h2>
            <UpdateSetTitle/>
            <br/>
            <UpdateSetLink/>
            <br/>
            </div>
        };
    } else {
        return html! {
        <CreateSet/>
        };
    }
}
#[function_component(UpdateSetLink)]
pub fn update_set_link() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if let Some(editable_set) = (edit_set_context.editable_set).clone() {
        let (set_uuid, title, collection_link) = editable_set.deconstruct();
        let title_ref = use_node_ref();
        let link_ref = use_node_ref();
        let update_link = {
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let edit_set_context = edit_set_context.clone();
            let link_ref = link_ref.clone();
            use_async::<_, (), String>(async move {
                match auth.update_link(set_uuid,link_ref.cast::<HtmlInputElement>().unwrap().value()).await? {
                    GraphQlResp::Data(data) => {
                        edit_set_context.dispatch(EditSetDataActions::NewEditFlag(true));
                        msg_context.dispatch(
                            new_green_msg_with_std_duration(data.update_link));
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
            <a href={collection_link.clone()}/>
            <input ref={link_ref.clone()}/>
            <button onclick={update_link.clone()}>{"Update Link"}</button>
            </div>
        };
    } else {
        return html! {
           <span>{"Error: No set to edit???"}</span>
        };
    }
}
#[function_component(UpdateSetTitle)]
pub fn update_set_title() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    if let Some(editable_set) = (edit_set_context.editable_set).clone() {
        let (set_uuid, title, collection_link) = editable_set.deconstruct();
        let title_ref = use_node_ref();
        let link_ref = use_node_ref();
        let update_title = {
            let token = auth_ctx.token.clone();
            let msg_context = msg_context.clone();
            let edit_set_ctx = edit_set_context.clone();
            let title_ref = title_ref.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<UpdateTitleMutation>(
                    update_title_mutation::Variables {
                        set_uuid: set_uuid.to_string(),
                        title: title_ref.cast::<HtmlInputElement>().unwrap().value(),
                    },
                    token,
                )
                .await
                .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    edit_set_ctx.dispatch(EditSetDataActions::NewEditFlag(true));
                    msg_context.dispatch(new_green_msg_with_std_duration("Updated".to_string()));
                } else if resp.errors.is_some() {
                    msg_context.dispatch(new_red_msg_with_std_duration(
                        map_graphql_errors_to_string(&resp.errors),
                    ));
                    tracing::error!("{:?}", resp.errors);
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
    } else {
        return html! {
            <span>{"Error: No set to edit???"}</span>
        };
    }
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
                edit_set_context.dispatch(EditSetDataActions::NewEditFlag(true));
                msg_context.dispatch(new_green_msg_with_std_duration("Set Created".to_string()))
            } else if resp.errors.is_some() {
                msg_context.dispatch(new_red_msg_with_std_duration(map_graphql_errors_to_string(
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
