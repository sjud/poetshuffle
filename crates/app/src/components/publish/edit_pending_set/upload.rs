use futures::{pin_mut, SinkExt, StreamExt};
use gloo::net::http::Headers;
use gloo::net::websocket::Message;
use js_sys::Uint8Array;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use yew_hooks::use_web_socket;
use super::*;
use shared::{FileType,TableCategory};
#[derive(Properties,PartialEq,Clone)]
pub struct UploadProps{
    pub(crate) file_ty:FileType,
    pub(crate) tab_cat:TableCategory,
    pub(crate) upload_msg:String,
    pub(crate) uuid:Uuid,
}

#[function_component(Upload)]
pub fn upload(props:&UploadProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let input_ref = use_node_ref();
    let upload = {
        let input_ref = input_ref.clone();
        let props = props.clone();
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        use_async::<_, (), ()>(async move {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            match auth.upload(props.tab_cat,props.file_ty,props.uuid,input)
                .await {
                Ok(()) => msg_context
                    .dispatch(
                        new_green_msg_with_std_duration("File uploaded.".into())),
                Err(err) => msg_context
                    .dispatch(
                        new_red_msg_with_std_duration(format!("{:?}",err)))
            }
            Ok(())
        })
    };
    let onchange = Callback::from(move|_|
        upload.run()
    );
    html!{
        <div>
        <form>
        <label for={props.uuid.to_string()}>{props.upload_msg.clone()}</label><br/>
        <input type="file" id={props.uuid.to_string()} ref={input_ref} {onchange}/>
        </form>
        </div>
    }
}
#[function_component(UploadWs)]
pub fn upload_ws(props:&UploadProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let input_ref = use_node_ref();
    let ws = use_web_socket(
        format!("ws://127.0.0.1:3000/api/upload_ws/{}",
                &auth_ctx.token.clone().unwrap(),
        ));
    let upload = {
        let input_ref = input_ref.clone();
        let props = props.clone();
        let ws = ws.clone();
        use_async::<_, (), ()>(async move {
            ws.send("Hello from ws client.".into());
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let bytes = Uint8Array::from(
                JsFuture::from(
                    input
                        .files()
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .array_buffer())
                    .await
                    .unwrap()).to_vec();
            tracing::error!("{:?}",bytes);
            ws.send_bytes(bincode::encode_to_vec(shared::UploadWsBinary {
                headers: shared::UploadHeaders {
                    file_ty: props.file_ty,
                    uuid: props.uuid,
                    table_cat: props.tab_cat
                },
                file: bytes,
            }, bincode::config::standard()).unwrap());
            Ok(())
        })
    };
    use_effect_with_deps(
        move |message| {
            if let Some(message) = &**message {
                msg_context.dispatch(
                    new_green_msg_with_std_duration(message.clone())
                )                    }
            || ()
        },
        ws.message,
    );
    let onchange = Callback::from(move|_|
        upload.run()
    );
    html!{
        <div>
        <form>
        <label for={props.uuid.to_string()}>{props.upload_msg.clone()}</label><br/>
        <input type="file" id={props.uuid.to_string()} ref={input_ref} {onchange}/>
        </form>
        </div>
    }
}