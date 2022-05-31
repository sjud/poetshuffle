use crate::components::{login::Login, main_menu::MainMenu};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/poetshuffle")]
    PoetShuffle,
    #[at("/about")]
    About,
    #[at("/login")]
    Login,
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
        Route::Login => html! {<Login/>},
        Route::Publish => html! {{"Publish"}},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
        //
    }
}
