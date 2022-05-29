use yew::{
    prelude::*,
    context::ContextProvider
};
use yew_router::prelude::*;
use crate::types::auth_context::{AuthToken,AuthContext};
use crate::routes::{Route,switch};
use crate::components::footer::Footer;
use crate::types::audio_context::{AudioOptions,AudioContext};

#[function_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let auth_token = use_reducer(||AuthToken::default());
    let audio_options = use_reducer(||AudioOptions::default());
    html! {
        <ContextProvider<AuthContext> context={auth_token}>
        <ContextProvider<AudioContext> context={audio_options}>
            <BrowserRouter>
            <div class ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        <Footer/>
                </div>
            </BrowserRouter>
        </ContextProvider<AudioContext>>
        </ContextProvider<AuthContext>>

    }
}

