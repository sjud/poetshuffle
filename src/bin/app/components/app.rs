use crate::components::footer::Footer;
use crate::routes::{switch, Route};
use crate::types::audio_context::{AudioContext, AudioOptions};
use crate::types::auth_context::{AuthContext, AuthToken};
use yew::{context::ContextProvider, prelude::*};
use yew_router::prelude::*;
use crate::types::footer_context::{FooterContext,FooterOptions};
use crate::types::mouse_move_context::{MouseMove,MouseMoveContext,MouseMoveActions};
use crate::types::msg_context::{UserMessageView,MsgContext};

#[function_component(App)]
pub fn app() -> Html {
    let render = Switch::render(switch);
    let auth_token = use_reducer(|| AuthToken::default());
    let audio_options = use_reducer(|| AudioOptions::default());
    let footer_options = use_reducer(|| FooterOptions::default());
    let user_msg = use_reducer(|| UserMessageView::default());
    let mouse_move = use_reducer(||MouseMove::default());
    let mouse_move_flag = use_state(||false);
    let onmousemove = {
        let mouse_move = mouse_move.clone();
        let mouse_move_flag = mouse_move_flag.clone();
        Callback::from(move |_| {
            if !(*mouse_move_flag) {
                mouse_move.dispatch(MouseMoveActions::MouseMoved);
                mouse_move_flag.set(true)
            }
        })
    };
    html! {
        <ContextProvider<AuthContext> context={auth_token}>
        <ContextProvider<AudioContext> context={audio_options}>
        <ContextProvider<FooterContext> context={footer_options}>
        <ContextProvider<MsgContext> context={user_msg}>
        <ContextProvider<MouseMoveContext> context={mouse_move}>
            <BrowserRouter>
            <div  {onmousemove}>
            <div class ="main">
                    <Switch<Route> {render} />
                </div>
                <div class="footer">
                        <Footer/>
                </div>
            </div>
            </BrowserRouter>
        </ContextProvider<MouseMoveContext>>
        </ContextProvider<MsgContext>>
        </ContextProvider<FooterContext>>
        </ContextProvider<AudioContext>>
        </ContextProvider<AuthContext>>


    }
}
