
mod publish;
mod admin;
mod poetshuffle;
mod services;
mod console_writer;

use std::rc::Rc;
use std::sync::Mutex;
use yew::prelude::*;
use yew::context::ContextProvider;
use yew_router::prelude::*;
use stylist::{yew::styled_component, css, global_style};
use crate::console_writer::WASMConsoleWriter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    console_error_panic_hook::set_once();
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_writer(Mutex::new(WASMConsoleWriter))
        .pretty()
        .init();
    yew::start_app::<App>();
}



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
fn switch(routes: &Route) -> Html {
    match routes {
        Route::PoetShuffle => html!{{"PoetShuffle"}},
        Route::About => html!{{"About"}},
        Route::Admin => html!{<admin::Admin/>},
        Route::Publish => html!{{"Publish"}},
        Route::MainMenu => html! {<MainMenu />},
        Route::NotFound => html! { {"404"}},
//
    }
}

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let history = use_history().unwrap();
    let poet_shuffle = Callback::from(move |_|history.push(Route::PoetShuffle));
    let menu_style = css!(
        r#"
          display: flex;
  height: 90vh;
  justify-content: center;
  align-items: center;
  text-align: center;
        "#);
    let menu_list = css!(r#"
  display: flex;
  flex-direction: column;
  align-items: start;
  list-style-type: none;
    "#);
    let menu_list_item = css!(r#"padding: 6px 0;"#);
    let menu_button = css!(r#"
  align-items: center;
  background-color: #fee6e3;
  border: 2px solid #111;
  border-radius: 8px;
  box-sizing: border-box;
  color: #111;
  cursor: pointer;
  display: block;
  font-family: Inter,sans-serif;
  font-size: 16px;
  height: 48px;
  justify-content: center;
  line-height: 24px;
  max-width: 100%;
  padding: 0 25px;
  position: relative;
  text-align: center;
  text-decoration: none;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
    :after {
  background-color: #111;
  border-radius: 8px;
  content: "";
  display: block;
  height: 48px;
  left: 0;
  width: 100%;
  position: absolute;
  top: -2px;
  transform: translate(8px, 8px);
  transition: transform .2s ease-out;
  z-index: -1;
    }

        :hover:after {
  transform: translate(0, 0);
}

:active {
  background-color: #D8BFD8;
  outline: 0;
}

:hover {
  outline: 0;
}

@media (min-width: 768px) {
  .button-56 {
    padding: 0 40px;
  }
}"#);
    html! {
        <div>
        <div class={menu_style}>
        <ul class={menu_list}>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Press"}</button>
        </li>
            <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"PoetShuffle"}</button>
        </li>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Discover"}</button>
        </li>
                    <li class = {menu_list_item.clone()}>
        <button onclick={poet_shuffle.clone()} class = {menu_button.clone()}>{"Poetry"}</button>
        </li>

        </ul>
        </div>
        </div>
    }

}

#[function_component(Footer)]
pub fn footer() -> Html {
    let history = use_history().unwrap();
    let about = Callback::from(move |_| history.push(Route::About));
    let history = use_history().unwrap();
    let admin = Callback::from(move |_| history.push(Route::Admin));
    let list = css!(r#"
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
"#);
    let button = css!(r#"
      align-items: center;
  justify-content: center;
  position: relative;
  text-align: center;
  background: none!important;
  border: none;
  padding: 0!important;
  font-family: Inter,sans-serif;
  color: black;
  cursor: pointer;

"#);

    html!{
        <div class={list}>
        <button onclick={about} class = {button.clone()}>{"About"}</button>
        <button onclick={admin} class = {button.clone()}>{"Admin"}</button>
        </div>
    }
}




#[styled_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let auth_token = use_reducer(||AuthToken::default());
    html! {
        <ContextProvider<AuthContext> context={auth_token}>

            <BrowserRouter>
            <div class ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        <Footer/>
                </div>
            </BrowserRouter>
        </ContextProvider<AuthContext>>

    }
}
pub type AuthContext = UseReducerHandle<AuthToken>;
#[derive(Default,PartialEq,Clone)]
pub struct AuthToken{
    token:String,
}

pub enum AuthTokenAction{
    Set(String)
}
impl Reducible for AuthToken {
    type Action = AuthTokenAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthTokenAction::Set(token) => {
                tracing::error!("AuthToken set: \n{}",token);
                Self{token}.into()}
        }
    }
}
