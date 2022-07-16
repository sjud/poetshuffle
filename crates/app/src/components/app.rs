use crate::components::footer::Footer;
use crate::routes::{switch, Route};
use crate::types::audio_context::{AudioContext, AudioOptions};
use crate::types::auth_context::{AuthContext, AuthToken};
use crate::types::footer_context::{FooterContext, FooterOptions};
use crate::types::msg_context::{MsgContext, UserMessageView};
use crate::types::transcript_context::{TranscriptOptions,TranscriptContext};
use yew::{context::ContextProvider, prelude::*};
use yew_router::prelude::*;
use crate::components::audio::audio_player::AudioPlayer;

pub const MAX_SIZE: u32 = 10_485_760;
#[function_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let auth_token = use_reducer(|| AuthToken::default());
    let audio_options = use_reducer(|| AudioOptions::default());
    let transcript_options = use_reducer(||TranscriptOptions::default());
    let footer_options = use_reducer(|| FooterOptions::default());
    let user_msg = use_reducer(|| UserMessageView::default());

    html! {
        <ContextProvider<AuthContext> context={auth_token}>
        <ContextProvider<AudioContext> context={audio_options}>
        <ContextProvider<FooterContext> context={footer_options}>
        <ContextProvider<MsgContext> context={user_msg}>
        <ContextProvider<TranscriptContext> context={transcript_options}>
            <BrowserRouter>
            <div class ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        <Footer/>
                </div>
            <AudioPlayer/>
            </BrowserRouter>
        </ContextProvider<TranscriptContext>>
        </ContextProvider<MsgContext>>
        </ContextProvider<FooterContext>>
        </ContextProvider<AudioContext>>
        </ContextProvider<AuthContext>>


    }
}
