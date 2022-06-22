mod add_poem;

use super::*;
use crate::queries::{poem_query, PoemQuery};
use add_poem::AddPoem;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
        let token = auth_ctx.token.clone();
        let msg_context = msg_context.clone();
        if let Some(true) = edit_set_context.poem_list_data.poem_load_flags.get(&uuid) {
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<PoemQuery>(
                    poem_query::Variables {
                        poem_uuid: uuid.to_string(),
                    },
                    token.clone(),
                )
                .await
                .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
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
                    // If we have no data then see if we have errors and print those to console.
                    else if resp.errors.is_some() {
                        msg_context.dispatch(new_red_msg_with_std_duration(
                            map_graphql_errors_to_string(&resp.errors),
                        ));
                        tracing::error!("{:?}", resp.errors);
                    }
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
