use yew::prelude::*;
use stylist::{css};
use yew_router::{
    hooks::use_history,
    prelude::History
};
use crate::routes::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().unwrap();
    let about = Callback::from(move |_| history.push(Route::About));
    let history = use_history().unwrap();
    let admin = Callback::from(move |_| history.push(Route::Admin));
    let list = css!(r#"
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
"#);
    let button = css!(r#"
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

"#);

    html!{
        <div class={list}>
        <button onclick={about} class = {button.clone()}>{"About"}</button>
        <button onclick={admin} class = {button.clone()}>{"Admin"}</button>
        </div>
    }
}
