use yew::prelude::*;
use stylist::{css};
use yew_router::{
    hooks::use_history,
    prelude::History
};
use crate::routes::Route;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let history = use_history().unwrap();
    let poet_shuffle = Callback::from(move |_|history.push(Route::PoetShuffle));
    let menu_style = css!(
        r#"
          display: flex;
  height: 90vh;
  justify-content: center;
  align-items: center;
  text-align: center;
        "#);
    let menu_list = css!(r#"
  display: flex;
  flex-direction: column;
  align-items: start;
  list-style-type: none;
    "#);
    let menu_list_item = css!(r#"padding: 6px 0;"#);
    let menu_button = css!(r#"
  align-items: center;
  background-color: #fee6e3;
  border: 2px solid #111;
  border-radius: 8px;
  box-sizing: border-box;
  color: #111;
  cursor: pointer;
  display: block;
  font-family: Inter,sans-serif;
  font-size: 16px;
  height: 48px;
  justify-content: center;
  line-height: 24px;
  max-width: 100%;
  padding: 0 25px;
  position: relative;
  text-align: center;
  text-decoration: none;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
    :after {
  background-color: #111;
  border-radius: 8px;
  content: "";
  display: block;
  height: 48px;
  left: 0;
  width: 100%;
  position: absolute;
  top: -2px;
  transform: translate(8px, 8px);
  transition: transform .2s ease-out;
  z-index: -1;
    }

        :hover:after {
  transform: translate(0, 0);
}

:active {
  background-color: #D8BFD8;
  outline: 0;
}

:hover {
  outline: 0;
}

@media (min-width: 768px) {
  .button-56 {
    padding: 0 40px;
  }
}"#);
    html! {
        <div>
        <div class={menu_style}>
        <ul class={menu_list}>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Press"}</button>
        </li>
            <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"PoetShuffle"}</button>
        </li>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Discover"}</button>
        </li>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Poetry"}</button>
        </li>

        </ul>
        </div>
        </div>
    }

}