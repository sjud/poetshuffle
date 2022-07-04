use crate::services::network::GraphQlResp;
use crate::types::edit_poem_context::{EditPoemAction, EditPoemContext, EditPoemData};
use crate::types::edit_poem_list_context::EditPoemListContext;
use super::*;


#[derive(Properties,PartialEq,Debug,Clone)]
pub struct EditPoemProps{
    uuid:Uuid,
}

#[function_component(EditPoem)]
pub fn edit_poem(props:&EditPoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    let edit_poem_ctx = use_state(||use_reducer(||EditPoemData::default()));
    if use_is_first_mount() {
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let edit_poem_ctx = edit_poem_ctx.clone();
            let poem_uuid = props.uuid;
        let set_uuid = edit_set_ctx.editable_set.as_ref().unwrap().set_uuid;
            use_async::<_, (), String>(async move {
                match auth.poem_query(poem_uuid).await? {
                    GraphQlResp::Data(data) => {
                        let poem = &data.poem.unwrap();
                        edit_poem_ctx.set(use_reducer(||
                            EditPoemData{
                                poem_uuid,
                                set_uuid,
                                banter_uuid: None,
                                title: "".to_string()
                            })
                        );
                    },
                    GraphQlResp::Err(errors) => {
                        msg_context.dispatch(errors.into_msg_action());
                    }
                };
                Ok(())
            }).run();
    };
    return html! {
                <ContextProvider<EditPoemContext> context={(*edit_poem_ctx).clone()}>
            <div>
        <h2>{"Edit Poem"}</h2>
            <UpdatePoemTitle/>
            <br/>
            //<UpdatePoemIdx/>
            <br/>
            </div>
        </ContextProvider<EditPoemContext>>
    };
}

#[function_component(UpdatePoemTitle)]
pub fn update_poem_title() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_poem_ctx = use_context::<EditPoemContext>().unwrap();
    let title_ref = use_node_ref();
    let update_title = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_poem_ctx = edit_poem_ctx.clone();
        let title_ref = title_ref.clone();
        use_async::<_, (), String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_poem(
                edit_poem_ctx.poem_uuid,
                None,
                Some(title.clone()),
                None,
                None, ).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_ctx.dispatch(EditPoemAction::UpdateTitle(title));
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(data.update_poem));
                },
                GraphQlResp::Err(errors) => msg_context
                    .dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let update_title = Callback::from(move |_| {
        update_title.run();
    });
    let title = edit_poem_ctx.title.clone();
    return html! {
            <div>
            <h4>{title}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            </div>
    };
}
/*
#[function_component(UpdatePoemIdx)]
pub fn update_poem_idx() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let title_ref = use_node_ref();
    let update_title = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_poem_list_ctx = edit_poem_list_ctx.clone();
        let title_ref = title_ref.clone();
        use_async::<_, (), String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_poem(
                edit_poem_ctx.poem_uuid,
                None,
                Some(title.clone()),
                None,
                None, ).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_list_ctx.dispatch(EditPoemAction::UpdateTitle(title));
                    msg_context.dispatch(
                        new_green_msg_with_std_duration(data.update_poem));
                },
                GraphQlResp::Err(errors) => msg_context
                    .dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let update_title = Callback::from(move |_| {
        update_title.run();
    });
    return html! {
            <div>
            <h4>{Title}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            </div>
    };
}*/