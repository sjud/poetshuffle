use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, css, global_style, style};
fn main() { yew::start_app_as_body::<App>(); }


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/poetshuffle")]
    PoetShuffle,
    #[at("/about")]
    About,
    #[at("/publish")]
    Publish,
    #[at("/")]
    MainMenu,
    #[not_found]
    #[at("/404")]
    NotFound,
}
fn switch(routes: &Route) -> Html {
    match routes {
        Route::PoetShuffle => html!{{"PoetShuffle"}},
        Route::About => html!{{"About"}},
        Route::Publish => html!{{"Publish"}},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
//
    }
}

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let history = use_history().unwrap();
    let history_1 = history.clone();
    let poet_shuffle = Callback::from(move |_|history.push(Route::PoetShuffle));
    let about = Callback::from(move |_| history_1.push(Route::About));
    let style = style!{

    };
    html! {
        <div class="main_menu">
        <button onclick={poet_shuffle} class = "button">{"PoetShuffle"}</button><br/>
        <button onclick={about}  class = "button">{"About"}</button><br/>
        </div>
    }

}


#[styled_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let style = global_style!(r#"
    body{
    background-color: #D8BFD8;
    }
    "#);
    html! {
            <BrowserRouter>
                <div main ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        {"Footer"}
                </div>
            </BrowserRouter>
    }
}

