use futures::StreamExt;
use js_sys::Uint8Array;
use yew_hooks::use_mount;
use wasm_streams::ReadableStream;
use wasm_bindgen::{JsCast,UnwrapThrowExt};
use super::*;

#[function_component(TextReader)]
pub fn reader() -> Html {
    let msg_context  = use_context::<MsgContext>().unwrap();
    let transcript_ctx = use_context::<TranscriptContext>().unwrap();
    let req = {
        let src = transcript_ctx.src.clone();
        let ts_ctx = transcript_ctx.clone();
        use_async::<_,(),()>(async move {
            if let Some(src) = src {
                match gloo::net::http::Request::get(&*src)
                    .send().await {
                    Ok(resp) => {
                        if let Some(body) = resp.body() {
                            // body.to_string() -> JsString which impl From<String>
                            let mut data = Vec::new();
                            let mut stream = ReadableStream::from_raw(
                                body
                                    .dyn_into()
                                    .unwrap_throw()
                            ).into_stream();
                            while let Some(Ok(chunk)) = stream.next().await {
                                data.push(chunk
                                    .dyn_into::<Uint8Array>()
                                    .unwrap_throw()
                                    .to_vec());
                            }
                            ts_ctx.dispatch(
                                TranscriptActions::UpdateText(Some(
                                    String::from_utf8(
                                        data
                                            .into_iter()
                                            .flatten()
                                            .collect()
                                    ).unwrap()
                                ))
                            );
                        } else {
                            msg_context.dispatch(
                                new_red_msg_with_std_duration(
                                    "Server error: Transcript response lacks body.".into()
                                )
                            )
                        }
                    }
                    Err(err) => msg_context.dispatch(
                        new_red_msg_with_std_duration(
                            format!("{:?}",err)
                        )
                    )
                }
            } else {
                msg_context.dispatch(
                    new_red_msg_with_std_duration(
                        "No transcript available.".into()
                    )
                )
            }
            Ok(())
        })
    };
    use_mount(move ||req.run());
    let text = transcript_ctx.text.clone()
        .unwrap_or("No transcript available.".into());
    let ts_ctx_clone = transcript_ctx.clone();
    let onclick = Callback::from(move |_|ts_ctx_clone.dispatch(
        TranscriptActions::MountReader(false)
    ));
    html!{
        <div>
        <p>{text.clone()}</p>
        <button {onclick}> {"Close"}</button>
        </div>
    }
}
