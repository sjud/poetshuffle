use super::*;

#[function_component(UpdatePoemTitle)]
pub fn update_poem_title(props:&PoemProps) -> Html {
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let title_ref = use_node_ref();
    let update_title = {
        let auth = auth_ctx.clone();
        let poem_uuid = props.uuid;
        let title_ref = title_ref.clone();
        use_async::<_,(),String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_poem(
                poem_uuid,
                None,
                Some(title.clone()),
            ).await? {
                GraphQlResp::Data(data) => {
                    let poem_data = edit_poem_list_ctx
                        .find_by_poem_uuid(poem_uuid)
                        .unwrap();
                    edit_poem_list_ctx.dispatch(EditPoemListAction::UpdatePoemData(
                        PoemData{ title, ..poem_data }));
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(data.update_poem)
                    );
                },
                GraphQlResp::Err(errors) => {
                    msg_ctx.dispatch(errors.into_msg_action());
                }
            }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        update_title.run()
    });
    html!{
            <div>
            <input ref={title_ref.clone()}/>
            <button {onclick}>{"Update Title"}</button>
            </div>
    }
}