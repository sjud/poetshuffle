use crate::routes::Route;
use stylist::css;
use yew::prelude::*;
use yew_router::{hooks::use_history, prelude::History};
use crate::styles::{main_menu_button, main_menu_list, main_menu_style};
use crate::types::footer_context::{FooterContext, FooterForm, FooterOptionsActions};

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let footer_ctx = use_context::<FooterContext>().unwrap();
    footer_ctx.dispatch(FooterOptionsActions::Transform(FooterForm::HomePage));

    let history = use_history().unwrap();
    let poet_shuffle = Callback::from(move |_| history.push(Route::PoetShuffle));
    let menu_style = main_menu_style();
    let menu_list = main_menu_list();
    let menu_list_item = css!(r#"padding: 6px 0;"#);
    let menu_button = main_menu_button();
    let text = css!(
        r#"  font-weight: bold;
  color:black;"#
    ); //

    html! {
        <div>
        <div class={menu_style}>
        <ul class={menu_list}>

            <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>
            <span class={text}>{"PoetShuffle"}</span>
        </button>
        </li>
        </ul>
        </div>
        </div>
    }
}
