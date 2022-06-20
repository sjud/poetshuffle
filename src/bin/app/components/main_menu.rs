use std::sync::Mutex;
use crate::routes::Route;
use stylist::css;
use web_sys::HtmlParagraphElement;
use yew::prelude::*;
use yew_hooks::use_is_first_mount;
use yew_router::{hooks::use_history, prelude::History};
use crate::styles::{button, main_menu_button, main_menu_list, main_menu_style};
use crate::types::auth_context::{AuthContext,UserRole};
use crate::types::footer_context::{FooterContext, FooterForm, FooterOptionsActions};
use crate::types::mouse_move_context::MouseMoveContext;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let footer_ctx = use_context::<FooterContext>().unwrap();
    footer_ctx.dispatch(FooterOptionsActions::Transform(FooterForm::HomePage));
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let history = use_history().unwrap();
    let poet_shuffle = Callback::from(move |_| history.push(Route::PoetShuffle));
    let history = use_history().unwrap();
    let admin = Callback::from(move |_| history.push(Route::Admin));
    let menu_style = main_menu_style();
    let menu_list = main_menu_list();
    let menu_list_item = css!(r#"padding: 6px 0;"#);
    let menu_button = main_menu_button();
    let text = css!(
        r#"  font-weight: bold;
  color:black;"#
    ); //
    let button = button();
    let instr_props1 = TypeInstructionProps{
        msg:"Press PoetShuffle".into()
    };
    let instr_props2 = TypeInstructionProps{
        msg:"Discover Poetry.".into()
    };
    html! {
        <div>
        <div class={menu_style}>
        <div>
        <TypeInstruction ..instr_props1/>
        <br/>
        <TypeInstruction ..instr_props2/>
        <ul class={menu_list}>

            <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>
            <span class={text}>{"PoetShuffle"}</span>
        </button>
        </li>
         if auth_ctx.user_role >= UserRole::Admin{
                    <li>
            <button onclick={admin.clone()} class = {button.clone()}>{"Admin"}</button>
            </li>
        }
        </ul>
        </div>
        </div>
        </div>
    }
}

#[derive(Properties,PartialEq)]
pub struct TypeInstructionProps{
    msg:String,
}
#[function_component(TypeInstruction)]
fn type_instruction(props:&TypeInstructionProps) -> Html {
    let text = use_state(||String::new());
    let stmt = props.msg.clone();
    let text_clone = text.clone();
    if use_is_first_mount()  {
        wasm_bindgen_futures::spawn_local(async move {
            let mut text_buf = String::new();
            for c in stmt.chars() {
                gloo::timers::future::TimeoutFuture::new(100).await;
                text_buf.push(c);
                text_clone.set(text_buf.clone());
            };
        });
    };
    html!{
        <div>
        <span>{(*text).clone()}</span>
        </div>
    }
}
