use crate::routes::Route;
use crate::styles::{green_user_msg, menu_list, red_user_msg, user_msg};
use crate::types::audio_context::AudioContext;
use crate::types::auth_context::{AuthContext, AuthTokenAction};
use crate::types::footer_context::{FooterContext, FooterForm};
use crate::types::msg_context::{MsgActions, MsgContext, MsgForm, MsgTheme};
use stylist::css;
use yew::prelude::*;
use yew_router::{hooks::use_history, prelude::History};
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
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let auth_ctx_clone = auth_ctx.clone();
    let history = use_history().unwrap();
    let logout = Callback::from(move |_| {
        auth_ctx.dispatch(AuthTokenAction::Set(None));
        //history.push(Route::MainMenu)
    });
    let base = match footer_ctx.form {
        FooterForm::HomePage => html! {
                    <div class={list.clone()}>
                    <button onclick={about} class = {button.clone()}>{"About"}</button>
                    <button onclick={publish} class = {button.clone()}>{"Publish"}</button>
                    <button class = {button.clone()}>{"Source"}</button>
                if auth_ctx_clone.token.is_none() {
                    <button onclick={admin} class = {button.clone()}>{"Login/Register"}</button>
                    } else {
                    <button onclick={logout} class = {button.clone()}>{"Logout"}</button>
                }
                    </div>
        },
        FooterForm::LoginPage => html! {
            <div class={list.clone()}>
            <button onclick={back} class = {button.clone()}>{"Back"}</button>
            </div>
        },
    };
    html! {
        <div class={list.clone()}>
        <UserMsg/>
        {base}
        </div>
    }
}
/// when we get a message
/// Display a message
/// and an x button to delete message
/// if we get a new message overwrite previous message
/// if we change pages delete current message <- logic in msg_context.
/// display some messages on a timer (500 errors)
#[function_component(UserMsg)]
pub fn user_msg_footer() -> Html {
    use gloo::timers::future::TimeoutFuture;
    use wasm_bindgen_futures::spawn_local;
    let msg_context = use_context::<MsgContext>().unwrap();
    let x = use_node_ref();
    // Haven't lifted return out of this because my IDE doesn't like it O.o
    if let Some(msg) = msg_context.msg.clone() {
        let theme = match msg.theme {
            MsgTheme::Green => green_user_msg(),
            MsgTheme::Red => red_user_msg(),
        };
        let msg_css = user_msg();
        let white = css!(
            r#"
        color:white;
        "#
        );
        match msg.form {
            MsgForm::Standard => {
                return html! {
                    <div class={vec![theme.clone(),msg_css.clone()]}>
                    <button ref={x.clone()}>{"X"}</button>
                    <span class={white.clone()}>{msg.body}</span>
                    </div>
                };
            }
            MsgForm::WithDuration(duration) => {
                let msg_context_clone = msg_context.clone();
                spawn_local(async move {
                    // seconds to millis
                    TimeoutFuture::new(duration as u32 * 1000).await;
                    msg_context_clone.dispatch(MsgActions::Clear);
                });
                return html! {
                    <div class={vec![theme.clone(),msg_css.clone()]}>
                    <span class={white.clone()}>{msg.body}</span>
                    </div>
                };
            }
        }
    } else {
        return html! {};
    }
}
