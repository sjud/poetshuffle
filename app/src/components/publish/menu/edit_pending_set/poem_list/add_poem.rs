use super::*;



#[function_component(AddPoem)]
pub fn add_poem() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if let Some(editable_set) = edit_set_context.editable_set.clone() {
        let add_poem = {
            let token = auth_ctx.token.clone();
            let msg_context = msg_context.clone();
            let edit_set_context = edit_set_context.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<AddPoemMutation>(
                    add_poem_mutation::Variables {
                        set_uuid: editable_set.set_uuid.to_string(),
                        idx: edit_set_context.poem_list_data.poem_uuids.len() as i64
                    }, token)
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    edit_set_context.dispatch(EditSetDataActions::NewEditFlag(true));
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
        let add_poem = Callback::from(move |_| {
            add_poem.run();
        });
        return html! {
        <div>
        <h2>{"Add Poem to Set"}</h2>
            <button onclick={add_poem.clone()}>{"Add Poem"}</button>
        </div>
        }
    } else {
        return html!{}
    }
}