use super::*;

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
