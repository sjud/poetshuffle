
use super::*;
use crate::queries::{poem_query, PoemQuery};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use web_sys::HtmlSelectElement;
use crate::services::network::GraphQlResp;



#[function_component(PoemList)]
pub fn poem_list() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let ready_to_sort = use_state(||false);
    let fetch_html = edit_set_context.poem_uuids
        .clone()
        .into_iter()
        .map(|uuid|{
            let prop = PoemProps{uuid};
            html!{
               <PoemCardFetch ..prop/>
            }
        }).collect::<Html>();
    let html = edit_set_context
        .poem_data
        .clone()
        .into_iter()
        .map(|data|html!{<PoemCard ..data/>})
        .collect::<Html>();
    return html! {
        <div>
        {fetch_html}
        {html}
        </div>
    };
}

#[derive(Properties, PartialEq, Clone,Debug)]
pub struct PoemProps {
    pub uuid: Uuid,
}

#[derive(PartialEq,Properties,Default,Clone)]
pub struct PoemData {
    pub uuid:Uuid,
    pub title: String,
    pub idx: i32,
}
#[function_component(PoemCard)]
pub fn poem_card(props:&PoemData) -> Html {
    html! {
        <div id={"PoemCard"} key={props.uuid.to_string()}>
        <p>{props.title.clone()}</p>
        <UpdatePoemIdx ..{UpdatePoemIdxProps{idx:props.idx}}/>
        <UpdatePoemTitle ..{PoemProps{uuid:props.uuid}}/>
        </div>
    }
}
#[function_component(PoemCardFetch)]
pub fn poem_card_fetch(props: &PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    if use_is_first_mount() {
        {
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let edit_set_ctx = edit_set_ctx.clone();
            let uuid = props.uuid;
            use_async::<_, (), String>(async move {
                match auth.poem_query(uuid).await? {
                    GraphQlResp::Data(data) => {
                        let poem = &data.poem.unwrap();
                        edit_set_ctx.dispatch(EditSetDataActions::InsertPoemData(PoemData{
                            uuid,
                            title: poem.title.clone(),
                            idx: poem.idx as i32,
                        }));
                    },
                    GraphQlResp::Err(errors) => {
                        msg_context.dispatch(errors.into_msg_action());
                    }
                };
                Ok(())
            })
        }.run();
    };
    html! {}
}

#[derive(Properties,PartialEq)]
pub struct UpdatePoemIdxProps {
    idx:i32
}
#[function_component(UpdatePoemIdx)]
pub fn update_poem_idx(props:&UpdatePoemIdxProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>()
        .unwrap();
    let set_uuid = edit_set_ctx.editable_set.as_ref().unwrap().set_uuid;
    let list_len = edit_set_ctx.poem_uuids.len();
    let poem_a_idx = props.idx;
    let select_ref = use_node_ref();
    let select_swap_html = (0..list_len)
        .into_iter()
        .map(|i|
    html!{<option value={i.to_string()}>{i}</option>})
        .collect::<Html>();
    let swap = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_set_ctx = edit_set_ctx.clone();
        let select_ref = select_ref.clone();
        use_async::<_, (), String>(async move {
            // We only cast value when updated.
            let poem_b_idx = select_ref.cast::<HtmlSelectElement>().unwrap().value();
            match auth.update_poem_idx(
                set_uuid,
                poem_a_idx as i64,
                i64::from_str(&poem_b_idx)
                    .map_err(|err|format!("{:?}",err))?).await? {
                GraphQlResp::Data(data) => {
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
        {"Order:" }{poem_a_idx}{" swap to : "}
        <select ref={select_ref.clone()}>{select_swap_html}</select>
        </span>
        <button onclick={swap.clone()}>{"Swap Order"}</button>
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
        let title_ref = title_ref.clone();
        let uuid = props.uuid;
        let title_state = title.clone();
        use_async::<_, (), String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_poem(
                uuid,
                None,
                Some(title.clone()),
                None,
                None,).await? {
                GraphQlResp::Data(_) => {
                    title_state.set(title);
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
            <h4>{(*title).clone()}</h4>
            <input ref={title_ref.clone()}/>
            <button onclick={update_title.clone()}>{"Update Title"}</button>
            </div>
        };
}
