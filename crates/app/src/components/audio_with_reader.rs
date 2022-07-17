use stylist::css;
use yew::prelude::*;
use crate::components::text_reader::TextReader;
use crate::components::audio::AudioPlayer;
use crate::types::transcript_context::TranscriptContext;

#[function_component(AudioWithReader)]
pub fn auid_with_reader() -> Html {
    let ts_ctx = use_context::<TranscriptContext>().unwrap();
    let css = css!(
        position:fixed;
        bottom:0px;
        width:320px;
    );
    let mount_reader = ts_ctx.mount_reader;
    html!{
        <div class={css}>
        {
            if mount_reader
            {html!{
                        <TextReader/>
            }} else {html!{}}
        }
        <AudioPlayer/>
        </div>
    }
}