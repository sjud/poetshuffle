use stylist::css;
use super::*;

#[function_component(AudioPlayer)]
pub fn audio_player() -> html {
    let css = css!(
        position:fixed;
        bottom:30px;
        width:85px;
    );
    let audio_ctx = use_context::<AudioContext>().unwrap();
    if let Some(src) = audio_ctx.src.clone() {
        html!{
        <audio controls=true autoplay=true {src} class={css}/>
        }
    } else {
        html!{
        <audio/>
        }
    }

}
