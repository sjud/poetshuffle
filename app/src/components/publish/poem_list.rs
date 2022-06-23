
use super::*;
use crate::queries::{poem_query, PoemQuery};
use add_poem::AddPoem;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::services::network::GraphQlResp;

#[function_component(PoemList)]
pub fn poem_list() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let poem_uuids = edit_set_context.poem_list_data.poem_uuids.clone();
    let poem_cards: UseStateHandle<Mutex<HashMap<Uuid, PoemProps>>> =
        use_state(|| Mutex::new(HashMap::new()));
    for uuid in poem_uuids {
        let poem_cards = poem_cards.clone();
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        if let Some(true) = edit_set_context.poem_list_data.poem_load_flags.get(&uuid) {
            use_async::<_, (), String>(async move {
                match auth.poem_query(uuid).await? {
                    GraphQlResp::Data(data) => {
                        if let Some(poem) = &data.poem {
                            // Turn use_state into Mutex, lock it unwrap to get the guard.
                            let mut poem_map = (*poem_cards).lock().unwrap();
                            poem_map.insert(
                                uuid,
                                PoemProps {
                                    title: poem.title.clone(),
                                    uuid,
                                    idx: poem.idx as i32,
                                },
                            );
                            poem_cards.set(Mutex::new(poem_map.clone()));
                        }
                    },
                    GraphQlResp::Err(errors) =>
                        msg_context.dispatch(errors.into_msg_action()),
                }
                Ok(())
            });
        }
    }
    let mut poem_props: Vec<PoemProps> = poem_cards.lock().unwrap().clone().into_values().collect();
    poem_props.sort_by(|prop_a, prop_b| Ord::cmp(&prop_a.idx, &prop_b.idx));
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
        <AddPoem/>
        </div>
    };
}
#[derive(Properties, PartialEq, Clone)]
pub struct PoemProps {
    pub title: String,
    pub uuid: Uuid,
    pub idx: i32,
}
#[function_component(PoemCard)]
pub fn poem_card(props: &PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let poem_title = use_state(|| String::new());
    let poem_idx = use_state(|| 0);

    html! {}
}
