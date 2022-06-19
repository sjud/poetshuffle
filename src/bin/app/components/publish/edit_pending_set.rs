use super::*;

#[derive(Default,PartialEq,Clone)]
pub struct EditableSet{
    pub set_uuid: Uuid,
    pub collection_title: String,
    pub collection_link: String,
}

#[derive(Properties,PartialEq)]
pub struct EditPendingSetProps{
    pub new_edit_flag:UseStateHandle<bool>,
    pub editable_set:UseStateHandle<Option<EditableSet>>,
    pub poem_uuids:UseStateHandle<Option<Vec<Uuid>>>,
}
#[function_component(EditPendingSet)]
pub fn edit_pending_set(props:&EditPendingSetProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let create_set = {
        let token = auth_ctx.token.clone();
        let msg_context = msg_context.clone();
        let new_edit_flag = props.new_edit_flag.clone();
        use_async::<_, (), String>(async move {
            let resp = post_graphql::<CreatePendingSetMutation>(
                create_pending_set_mutation::Variables {}, token)
                .await
                .map_err(|err| format!("{:?}", err))?;
            if let Some(ref data) = resp.data {
                new_edit_flag.set(true);
                msg_context.dispatch(
                    new_green_msg_with_std_duration(
                        "Set Created".to_string())
                )
            } else if resp.errors.is_some() {
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

    let create_set = Callback::from(move |_| {
        create_set.run();
    });

    if let Some(editable_set) = (*props.editable_set).clone(){
        let (set_uuid,title,collection_link) = (
            editable_set.set_uuid.clone(),
            editable_set.collection_title.clone(),
            editable_set.collection_link.clone()
        );
        let title_ref = use_node_ref();
        let link_ref = use_node_ref();
        let update_title = {
            let token = auth_ctx.token.clone();
            let msg_context = msg_context.clone();
            let new_edit_flag = props.new_edit_flag.clone();
            let title_ref = title_ref.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<UpdateTitleMutation>(
                    update_title_mutation::Variables {
                        set_uuid:set_uuid.to_string(),
                        title:title_ref.cast::<HtmlInputElement>().unwrap().value()
                    }, token)
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    new_edit_flag.set(true);
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(
                            "Updated".to_string()
                        ));
                } else if resp.errors.is_some() {
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
        let update_title = Callback::from(move|_| {
            update_title.run();
        });
        let update_link = {
            let token = auth_ctx.token.clone();
            let msg_context = msg_context.clone();
            let new_edit_flag = props.new_edit_flag.clone();
            let link_ref = link_ref.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<UpdateLinkMutation>(
                    update_link_mutation::Variables {
                        set_uuid:set_uuid.to_string(),
                        link:link_ref.cast::<HtmlInputElement>().unwrap().value()
                    }, token)
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    new_edit_flag.set(true);
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(
                            "Updated".to_string()
                        ));
                } else if resp.errors.is_some() {
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
        let update_link = Callback::from(move|_| {
            update_link.run();
        });
        let add_poem = {
            let token = auth_ctx.token.clone();
            let msg_context = msg_context.clone();
            let new_edit_flag = props.new_edit_flag.clone();
            let poem_uuids = props.poem_uuids.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<AddPoemMutation>(
                    add_poem_mutation::Variables {
                        set_uuid:set_uuid.to_string(),
                        idx:(*poem_uuids).as_ref().unwrap_or(&Vec::new()).len() as i64
                    }, token)
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    new_edit_flag.set(true);
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(
                            "Updated".to_string()
                        ));
                } else if resp.errors.is_some() {
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
        let add_poem = Callback::from(move|_| {
            add_poem.run();
        });
        // We use the title we got pass from props.
        return html!{
            <div>
        <h2>{"Edit Pending Set"}</h2>
            <h3>{"Title:"}</h3>
            <h4>{title}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            <br/>
            <h3>{"Link:"}</h3>
            <a href={collection_link.clone()}/>
            <input ref={link_ref.clone()}/>
            <button onclick={update_link.clone()}>{"Update Link"}</button>
            <br/>
            <h2>{"Add Poem to Set"}</h2>
            <button onclick={add_poem.clone()}>{"Add Poem"}</button>
            </div>
        };
    } else {
        return html! {
        <button onclick={create_set.clone()}>{"Create New Set"}</button>
        }
    }
}

