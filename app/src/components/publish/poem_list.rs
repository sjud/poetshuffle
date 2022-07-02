
use super::*;
/*
use crate::queries::{poem_query, PoemQuery};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::services::network::GraphQlResp;

#[function_component(PoemList)]
pub fn poem_list() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let poem_uuids = edit_set_context.poem_uuids.clone();
    let poem_cards: UseStateHandle<HashMap<Uuid, PoemProps>> = use_state(|| HashMap::new());
    let first_fetch = use_state(||true);
    let unfinished_fetch_count = use_state(||poem_uuids.len());
    let map: Arc<Mutex<HashMap<Uuid, PoemProps>>> = Arc::new(Mutex::new(HashMap::new()));
    if *first_fetch && !poem_uuids.is_empty() {
        for uuid in poem_uuids {
            let edit_set_context = edit_set_context.clone();
            let auth = auth_ctx.clone();
            let msg_context = msg_context.clone();
            let map = map.clone();
            let unfinished_fetch_count =
                Arc::new(Mutex::new(unfinished_fetch_count.clone()));
            use_async::<_, (), String>(async move {
                    match auth.poem_query(uuid).await? {
                        GraphQlResp::Data(data) => {
                            let poem = &data.poem.unwrap();
                            map.lock()
                                .unwrap()
                                .insert(uuid, PoemProps {
                                    title: poem.title.clone(),
                                    uuid,
                                    idx: poem.idx as i32,
                                });
                        },
                        GraphQlResp::Err(errors) => {
                            msg_context.dispatch(errors.into_msg_action());
                        }
                    };
                let lock = (*unfinished_fetch_count).lock().unwrap();
                let count = **lock;
                lock.set(count-1);
                    Ok(())
                }).run();
        }
        first_fetch.set(false);
    } else {
        unfinished_fetch_count.set(0);
    }
    if *unfinished_fetch_count == 0 {
        poem_cards.set((*map).lock().unwrap().clone());
        let mut poem_props: Vec<PoemProps> = (*poem_cards)
            .clone()
            .into_values().collect();
        poem_props.sort_by(
            |prop_a, prop_b|
                Ord::cmp(&prop_a.idx, &prop_b.idx));
        let poem_card_html = poem_props
            .into_iter()
            .map(|prop| {
                html! {
                <div key={prop.uuid.as_u128()}>
                <PoemCard ..prop/>
                </div>
            }
            })
            .collect::<Html>();

        return html! {
        <div>
        {poem_card_html}
        </div>
    };
    } else {
        return html!{
            <p>{"loading..."}</p>
        }
    }

}
#[derive(Properties, PartialEq, Clone,Debug)]
pub struct PoemProps {
    pub title: String,
    pub uuid: Uuid,
    pub idx: i32,
}

#[function_component(PoemCard)]
pub fn poem_card(props: &PoemProps) -> Html {

    html! {<p>{"Poem card."}</p>}
}
*/