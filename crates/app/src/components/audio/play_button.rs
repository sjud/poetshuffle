use super::*;


#[derive(Properties,PartialEq,Clone,Debug)]
pub struct PlayButtonProps{
    pub uuid:Uuid,
    pub tab_cat:TableCategory,
}
#[tracing::instrument]
#[function_component(PlayButton)]
pub fn play_button(props:&PlayButtonProps) -> Html {
    let audio_ctx = use_context::<AudioContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let req = {
        let props = props.clone();
        use_async::<_,(),String>(async move {
            match auth_ctx.presigned_url(props.tab_cat,FileType::Audio,props.uuid).await {
                Ok(Some(src)) => {
                    audio_ctx.dispatch(AudioActions::SetSrc(Some(src)))
                },
                Ok(None) => msg_ctx.dispatch(
                    new_red_msg_with_std_duration(
                        "Can't find audio file, was one uploaded?".into()
                    )),
                Err(err) => {
                    tracing::error!("{:?}",err);
                    msg_ctx.dispatch(new_red_msg_with_std_duration(err.to_string()));
                },
            };
            Ok(())
        })
    };
    let onclick = Callback::from(move|_|req.run());
    html!{<button {onclick} >{"Play"}</button>}
}
