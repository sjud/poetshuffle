use crate::types::audio_context::AudioContext;
use yew::prelude::*;

#[function_component(PoetShuffle)]
fn poetshuffle() -> Html {
    let audio_context = use_context::<AudioContext>().expect("Expecting Audio Context.");

    html! {
        <div>
        <div>{"PoetImg"}</div>
        <div>{"PoetName"}</div>
        <div>{"PoemTitle"}</div>
        <div>{"CollectionTitle"}</div>
        <div>{"CollectionLink"}</div>
        </div>
    }
}
