use crate::routes::Route;
use crate::styles::{button, main_menu_button, main_menu_list, main_menu_style};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::footer_context::{FooterContext, FooterForm, FooterOptionsActions};
use std::sync::Mutex;
use stylist::css;
use web_sys::HtmlParagraphElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};
use yew_router::{hooks::use_history, prelude::History};

pub const INSTRUCTION_DURATION:u32 = 100;

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
    let instruction_one = "Press PoetShuffle";
    let mut instr_props1 = TypeInstructionProps {
        msg: instruction_one.into(),
        wait:0,

    };
    let instr_props2 = TypeInstructionProps {
        msg: "Discover Poetry".into(),
        wait:(instruction_one.len() as u32 + 4) * INSTRUCTION_DURATION

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
#[derive(Properties, PartialEq)]
pub struct TypeInstructionProps {
    msg: String,
    wait:u32,
}
#[function_component(TypeInstruction)]
fn type_instruction(props: &TypeInstructionProps) -> Html {

    let text = use_state(|| String::new());
    let stmt = props.msg.clone();
    let wait = props.wait;
    let text_clone = text.clone();
    if use_is_first_mount() {
        wasm_bindgen_futures::spawn_local(async move {
            gloo::timers::future::TimeoutFuture::new(wait).await;
            let mut text_buf = String::new();
            for c in stmt.chars() {
                gloo::timers::future::TimeoutFuture::new(INSTRUCTION_DURATION).await;
                if c == ' ' {
                    gloo::timers::future::TimeoutFuture::new(INSTRUCTION_DURATION).await;
                }
                text_buf.push(c);
                text_clone.set(text_buf.clone());
            }
        });
    };

    /// \u00A0 is nonbreaking space, it makes sure our span
    /// exists invisibly and that there is no spacing readjustments
    /// as text is types.
    html! {
        <div>
        <span>{"\u{00A0}"}{(*text).clone()}</span>
        </div>
    }
}