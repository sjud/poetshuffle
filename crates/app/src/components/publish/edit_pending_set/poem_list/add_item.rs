use super::*;


#[function_component(AddPoem)]
pub fn add_poem() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let title_ref = use_node_ref();
    let add_poem = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let poem_list_ctx = poem_list_ctx.clone();
        let editable_set = edit_set_context.editable_set.clone().unwrap();
        use_async::<_, (), String>(async move {
            let set_uuid = editable_set.set_uuid;
            match auth.add_poem(set_uuid)
                .await? {
                GraphQlResp::Data(data) => {
                    poem_list_ctx.dispatch(
                        EditPoemListAction::PushPoemData(
                            PoemData{
                                uuid: Uuid::from_str(&data.add_poem.poem_uuid).unwrap(),
                                title: String::new(),
                                idx: data.add_poem.idx,
                                banter_uuid: None,
                                set_uuid,
                                approved: false
                            }
                        ));
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
}

#[function_component(AddBanter)]
pub fn add_banter(props:&BanterProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let add_banter = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let poem_list_ctx = poem_list_ctx.clone();
        let props = props.clone();
        use_async::<_, (), String>(async move {
            match auth.add_banter(props.poem_props.uuid)
                .await? {
                GraphQlResp::Data(data) => {
                    poem_list_ctx.dispatch(
                        EditPoemListAction::UpdatePoemWithBanter{
                            poem_uuid: props.poem_props.uuid,
                            banter_uuid: Some(
                                Uuid::from_str(
                                    &data.add_banter.banter_uuid)
                                    .unwrap()),
                        });
                    msg_context.dispatch(
                        new_green_msg_with_std_duration("Banter Added".into()));
                },
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let onclick = Callback::from(move |_| {
        add_banter.run();
    });
    return html! {
        <div>
            <button {onclick}>{"Add Banter"}</button>
        </div>
    };
}