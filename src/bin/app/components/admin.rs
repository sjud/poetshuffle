use yew::prelude::*;
use crate::components::{
    login::Login,
};
#[function_component(Admin)]
pub fn admin() -> Html {
    html!{
        <div>
        <Login/>
        </div>
    }
}
