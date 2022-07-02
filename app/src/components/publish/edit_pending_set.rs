use crate::services::network::GraphQlResp;
use super::*;
use crate::types::edit_set_context::{EditableSet, EditSetContext, EditSetDataActions};
#[function_component(EditPendingSet)]
pub fn edit_pending_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if let Some(editable_set) = (edit_set_context.editable_set).clone() {
        let props = UpdateSetProps{editable_set};
        // We use the title we got pass from props.
        return html! {
            <div>
        <h2>{"Edit Pending Set"}</h2>
            <UpdateSetTitle ..props.clone()/>
            <br/>
            <UpdateSetLink ..props.clone()/>
            <br/>
            <AddPoem ..props.clone()/>
            </div>
        };
    } else {
        return html! {
        <CreateSet/>
        };
    }
}

#[derive(Properties,PartialEq,Clone)]
pub struct UpdateSetProps{
    editable_set:EditableSet,
}
// TODO The following two components are basically the same, make them one.
#[function_component(UpdateSetLink)]
pub fn update_set_link(props:&UpdateSetProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let editable_set = props.clone().editable_set;
    let (set_uuid, title, collection_link) = editable_set.deconstruct();
    let title_ref = use_node_ref();
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
                    edit_set_context.dispatch(EditSetDataActions::UpdateLink(link));
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
            <a href={collection_link.clone()}/>
            <input ref={link_ref.clone()}/>
            <button onclick={update_link.clone()}>{"Update Link"}</button>
            </div>
        };
}
#[function_component(UpdateSetTitle)]
pub fn update_set_title(props:&UpdateSetProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let editable_set = props.clone().editable_set;
    let (set_uuid, title, collection_link) = editable_set.deconstruct();
    let title_ref = use_node_ref();
    let link_ref = use_node_ref();
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
                    edit_set_ctx.dispatch(EditSetDataActions::UpdateTitle(title));
                    msg_context.dispatch(new_green_msg_with_std_duration("Updated".to_string()));
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
#[function_component(AddPoem)]
pub fn add_poem(props:&UpdateSetProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let title_ref = use_node_ref();
    let add_poem = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_set_context = edit_set_context.clone();
        let editable_set = props.editable_set.clone();
        use_async::<_, (), String>(async move {
            match auth.add_poem(
                editable_set.set_uuid,
                edit_set_context.poem_uuids.len() as i64)
                .await? {
                GraphQlResp::Data(data) => {
                    edit_set_context.dispatch(
                        EditSetDataActions::PushPoemUuid(
                            Uuid::from_str(&data.add_poem).unwrap()));
                    msg_context.dispatch(new_green_msg_with_std_duration("Poem Added".into()));
                },
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let add_poem = Callback::from(move |_| {
        add_poem.run();
    });
    return html! {
        <div>
        <h2>{"Add Poem to Set"}</h2>
            <button onclick={add_poem.clone()}>{"Add Poem"}</button>
        </div>
        };
    /*html!{
        <div>
         <h3>{"Title:"}</h3>
        <input ref={title_ref.clone()}/>
        <button>{"Choose File"}</button>
        <button>{"Add Poem"}</button>
        </div>
    }*/
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
