use crate::services::network::GraphQlResp;
use super::*;

#[function_component(AddPoem)]
pub fn add_poem() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();

    if let Some(editable_set) = edit_set_context.editable_set.clone() {
        let add_poem = {
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let edit_set_context = edit_set_context.clone();
            use_async::<_, (), String>(async move {
                match auth.add_poem(
                    editable_set.set_uuid,
                    edit_set_context.poem_list_data.poem_uuids.len() as i64)
                    .await? {
                    GraphQlResp::Data(data) => {
                        edit_set_context.dispatch(EditSetDataActions::NewEditFlag(true));
                        msg_context.dispatch(
                            new_green_msg_with_std_duration(
                            data.add_poem));
                    },
                    GraphQlResp::Err(errors) => msg_context
                        .dispatch(errors.into_msg_action())
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
    } else {
        return html! {};
    }
}
