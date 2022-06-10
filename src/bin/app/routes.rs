use crate::components::{login_register::LoginRegister, main_menu::MainMenu};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/poetshuffle")]
    PoetShuffle,
    #[at("/about")]
    About,
    #[at("/login_register")]
    LoginRegister,
    #[at("/publish")]
    Publish,
    #[at("/")]
    MainMenu,
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub(crate) fn switch(routes: &Route) -> Html {
    match routes {
        Route::PoetShuffle => html! {{"PoetShuffle"}},
        Route::About => html! {{"About"}},
        Route::LoginRegister => html! {<LoginRegister/>},
        Route::Publish => html! {{"Publish"}},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
        //
    }
}
