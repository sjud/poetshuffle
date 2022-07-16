use crate::components::validation::login::{Login, LoginProps};
use crate::components::validation::register::Register;
use crate::styles::login_register_style;
use crate::types::footer_context::{FooterContext, FooterForm, FooterOptionsActions};
use yew::prelude::*;

#[function_component(LoginRegister)]
pub fn login() -> Html {
    let footer_ctx = use_context::<FooterContext>().unwrap();
    footer_ctx.dispatch(FooterOptionsActions::Transform(FooterForm::LoginPage));
    let style = login_register_style();
    let login_props = LoginProps {
        super_admin_login: false,
    };
    html! {
        <div class={style.clone()}>
        <Register/>
        <Login ..login_props/>
        </div>
    }
}
