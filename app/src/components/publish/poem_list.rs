
use super::*;
use crate::queries::{poem_query, PoemQuery};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::services::network::GraphQlResp;

#[function_component(PoemList)]
pub fn poem_list() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let html = edit_set_context.poem_uuids
        .clone()
        .into_iter()
        .map(|uuid|{
            let prop = PoemProps{uuid};
            html!{
               <PoemCard ..prop/>
            }
        }).collect::<Html>();
    return html!{
        <div>
        {html}
        </div>
    };
}

#[derive(Properties, PartialEq, Clone,Debug)]
pub struct PoemProps {
    pub uuid: Uuid,
}

#[derive(PartialEq,Properties,Default)]
pub struct PoemData {
    title: String,
    idx: i32,
}

#[function_component(PoemCard)]
pub fn poem_card(props: &PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let data = use_state(||PoemData::default());
    if use_is_first_mount() {
        {
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let poem_data = data.clone();
            let uuid = props.uuid;
            use_async::<_, (), String>(async move {
                match auth.poem_query(uuid).await? {
                    GraphQlResp::Data(data) => {
                        let poem = &data.poem.unwrap();
                        poem_data.set(PoemData{
                            title: poem.title.clone(),
                            idx: poem.idx as i32,
                        });
                    },
                    GraphQlResp::Err(errors) => {
                        msg_context.dispatch(errors.into_msg_action());
                    }
                };
                Ok(())
            })
        }.run();
    };
    html! {
        <div key={data.idx}>
        <p>{data.title.clone()}</p>
        <p>{data.idx}</p>
        </div>
    }
}


#[function_component(UpdatePoemTitle)]
pub fn update_set_title(props:&PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let title_ref = use_node_ref();
    let title = use_state(||String::new());
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
                    edit_set_ctx.dispatch(EditSetDataActions::UpdateTitle(title));
                    msg_context.dispatch(new_green_msg_with_std_duration("Updated".to_string()));
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
            <h4>{title}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            </div>
        };
}
