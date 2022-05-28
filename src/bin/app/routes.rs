use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    admin::Admin,
    main_menu::MainMenu};


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/poetshuffle")]
    PoetShuffle,
    #[at("/about")]
    About,
    #[at("/admin")]
    Admin,
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
        Route::PoetShuffle => html!{{"PoetShuffle"}},
        Route::About => html!{{"About"}},
        Route::Admin => html!{<Admin/>},
        Route::Publish => html!{{"Publish"}},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
//
    }
}