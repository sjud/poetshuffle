use stylist::css;
use super::*;
#[function_component(AudioPlayer)]
pub fn audio_player() -> Html {
    let audio_ctx = use_context::<AudioContext>().unwrap();
    if let Some(src) = audio_ctx.src.clone() {
        return html!{
        <audio controls=true autoplay=true {src}/>
        };
    } else {
        return html!{<div>
            <audio/>
            </div>
        };
    }

}

