use super::*;


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