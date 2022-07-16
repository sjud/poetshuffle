use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::use_async;
use shared::{FileType, TableCategory};
use crate::types::auth_context::AuthContext;
use crate::types::msg_context::{MsgContext, new_red_msg_with_std_duration};
use crate::types::transcript_context::{TranscriptActions, TranscriptContext};

#[function_component(TextReader)]
pub fn reader() -> Html{
    let msg_context  = use_context::<MsgContext(>
    html!{{"Todo reader.."}}
}

#[derive(Properties,PartialEq)]
pub struct ReadButtonProps {
    pub(crate) tab_cat:TableCategory,
    pub(crate) uuid:Uuid,
}

#[function_component(ReadButton)]
pub fn read_button(props:&ReadButtonProps) -> Html {
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let transcript_ctx = use_context::<TranscriptContext>().unwrap();
    let req = {
        let (tab_cat,text_uuid) = (props.tab_cat,props.uuid);
        use_async::<_,(),()>(async move {
            match auth_ctx.presigned_url(tab_cat,FileType::Transcript,text_uuid).await {
                Ok(Some(src)) =>
                transcript_ctx.dispatch(TranscriptActions::SetSrc(Some(src))),
                Ok(None) =>
                msg_ctx.dispatch(
                    new_red_msg_with_std_duration("Transcript not found.".into())
                ),
                Err(err) =>
                msg_ctx.dispatch(
                    new_red_msg_with_std_duration(
                        err.to_string()
                    )
                ),
            }
            Ok(())
        })
    };
    let onclick = Callback::from(move |_|req.run());
    html!{
        <button {onclick}>{"Read"}</button>
    }
}