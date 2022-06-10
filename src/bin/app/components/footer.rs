use crate::routes::Route;
use crate::types::audio_context::{AudioContext, AudioOptions};
use stylist::css;
use yew::prelude::*;
use yew_router::{hooks::use_history, prelude::History};
use crate::styles::menu_list;
use crate::types::footer_context::{FooterContext, FooterForm};

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().unwrap();
    let about = Callback::from(move |_| history.push(Route::About));
    let history = use_history().unwrap();
    let admin = Callback::from(move |_| history.push(Route::LoginRegister));
    let history = use_history().unwrap();
    let publish = Callback::from(move |_| history.push(Route::Publish));
    let history = use_history().unwrap();
    let back = Callback::from(move |_| history.push(Route::MainMenu));
    let list = menu_list();
    let button = crate::styles::button();
    let footer_ctx = use_context::<FooterContext>().unwrap();
    let audio_ctx = use_context::<AudioContext>().unwrap();

    match footer_ctx.form {
        FooterForm::HomePage => html! {
                <div class={list.clone()}>
                <button onclick={about} class = {button.clone()}>{"About"}</button>
                <button onclick={publish} class = {button.clone()}>{"Publish"}</button>
                <button onclick={admin} class = {button.clone()}>{"Login/Register"}</button>
            if audio_ctx.is_visible {<AudioFooter/>}
                </div>
    },
        FooterForm::LoginPage => html!{
            <div class={list.clone()}>
            <button onclick={back} class = {button.clone()}>{"Back"}</button>
            if audio_ctx.is_visible {<AudioFooter/>}
            </div>

        },

    }

}

#[function_component(AudioFooter)]
pub fn audio_footer() -> Html {
    let audio_ctx = use_context::<AudioContext>().unwrap();
    html! {
    <div>
        <button>
        if !audio_ctx.is_playing{<button>{"Play"}</button>}
        else {<button>{"Pause"}</button>}
        </button>
        </div>
        }
}