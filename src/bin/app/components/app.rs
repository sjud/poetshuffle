use crate::components::footer::Footer;
use crate::routes::{switch, Route};
use crate::types::audio_context::{AudioContext, AudioOptions};
use crate::types::auth_context::{AuthContext, AuthToken};
use yew::{context::ContextProvider, prelude::*};
use yew_router::prelude::*;
use crate::types::footer_context::{FooterContext,FooterOptions};

#[function_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let auth_token = use_reducer(|| AuthToken::default());
    let audio_options = use_reducer(|| AudioOptions::default());
    let footer_options = use_reducer(|| FooterOptions::default());
    html! {
        <ContextProvider<AuthContext> context={auth_token}>
        <ContextProvider<AudioContext> context={audio_options}>
        <ContextProvider<FooterContext> context={footer_options}>
            <BrowserRouter>
            <div class ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        <Footer/>
                </div>
            </BrowserRouter>
        </ContextProvider<FooterContext>>
        </ContextProvider<AudioContext>>
        </ContextProvider<AuthContext>>


    }
}
