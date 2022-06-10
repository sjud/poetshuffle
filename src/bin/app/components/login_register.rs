use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::components::{login::Login,register::Register};
use crate::styles::login_register_style;
use crate::types::footer_context::{FooterContext, FooterForm, FooterOptionsActions};

#[function_component(LoginRegister)]
pub fn login() -> Html {
    let footer_ctx = use_context::<FooterContext>().unwrap();
    footer_ctx.dispatch(FooterOptionsActions::Transform(FooterForm::LoginPage));
    let style = login_register_style();
    html! {
        <div class={style.clone()}>
        <Register/>
        <Login/>
        </div>
    }
}
