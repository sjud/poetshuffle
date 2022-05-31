use crate::routes::Route;
use crate::types::audio_context::{AudioContext, AudioOptions};
use stylist::css;
use yew::prelude::*;
use yew_router::{hooks::use_history, prelude::History};

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().unwrap();
    let about = Callback::from(move |_| history.push(Route::About));
    let history = use_history().unwrap();
    let admin = Callback::from(move |_| history.push(Route::Login));
    let history = use_history().unwrap();
    let publish = Callback::from(move |_| history.push(Route::Publish));
    let list = css!(
        r#"
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
"#
    );
    let button = css!(
        r#"
      align-items: center;
  justify-content: center;
  position: relative;
  text-align: center;
  background: none!important;
  border: none;
  padding: 0!important;
  font-family: Inter,sans-serif;
  color: black;
  cursor: pointer;
  height: 5.5vh;
  :hover {text-decoration:underline;}
  :active {text-decoration:underline;}
"#
    );

    let white = css!(r#"color:white;"#);

    let normal_footer = html! {
        <div class={list.clone()}>
        <button onclick={about} class = {button.clone()}>{"About"}</button>
        <button onclick={publish} class = {button.clone()}>{"Publish"}</button>
        <button onclick={admin} class = {button.clone()}>{"Login/Register"}</button>
        </div>
    };
    let audio_ctx = use_context::<AudioContext>().unwrap();

    let audio_footer = html! {
        <div class={list}>
        <button>
        {if !audio_ctx.is_playing
            {"Play"} else {"Pause"}
        }</button>
        </div>
    };
    if audio_ctx.is_visible {
        audio_footer
    } else {
        normal_footer
    }
}
