use super::*;

#[function_component(UpdatePoemIdx)]
pub fn update_idx(props:&PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let poem = edit_poem_list_ctx.find_by_poem_uuid(props.uuid).unwrap();
    let set_uuid = poem.set_uuid;
    let poem_a_idx = poem.idx;
    let list_len = edit_poem_list_ctx.poems.len();
    let select_ref = use_node_ref();
    // We display indexes as 1 greater, but store true values in value attribute.
    let select_swap_html = (1..=list_len)
        .into_iter()
        .map(|i|
            html!{<option value={(i-1).to_string()}>{i}</option>})
        .collect::<Html>();
    let swap = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_poem_list_ctx = edit_poem_list_ctx.clone();
        let select_ref = select_ref.clone();
        use_async::<_, (), String>(async move {
            let poem_b_idx = select_ref.cast::<HtmlSelectElement>().unwrap().value();
            let poem_b_idx = i64::from_str(&poem_b_idx)
                .map_err(|err|format!("{:?}",err))?;
            match auth.update_poem_idx(
                set_uuid, poem_a_idx,poem_b_idx).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_list_ctx.dispatch(
                        EditPoemListAction::SwapIdx(poem_a_idx,poem_b_idx));
                    msg_context.dispatch(new_green_msg_with_std_duration(data.update_poem_idx));
                }
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let swap = Callback::from(move |_| {
        swap.run();
    });
    html!{
        <div>
        <span>
        {"Order:" }{poem_a_idx+1}{" swap to : "}
        <select ref={select_ref.clone()}>{select_swap_html}</select>
        </span>
        <button onclick={swap.clone()}>{"Swap Order"}</button>
        </div>
    }
}