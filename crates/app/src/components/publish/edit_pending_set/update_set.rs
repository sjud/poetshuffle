use super::*;

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
