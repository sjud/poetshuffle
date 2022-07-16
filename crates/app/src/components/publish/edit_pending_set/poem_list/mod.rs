mod add_item;
pub use add_item::*;
mod delete_item;
pub use delete_item::*;
mod approve_item;
pub use approve_item::*;
mod update_poem_title;
pub use update_poem_title::*;
mod update_poem_idx;
pub use update_poem_idx::*;
mod upload_item;
pub use upload_item::*;
mod poem;
pub use poem::*;
mod banter;
pub use banter::*;

use gloo::net::websocket::Message;
use js_sys::Uint8Array;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::HtmlSelectElement;
use crate::components::audio::{PlayButtonProps,PlayButton};
use crate::services::network::{GraphQlResp};
use crate::types::edit_poem_list_context::{EditPoemListAction, EditPoemListContext, PoemData};
use crate::components::publish::*;
use futures::{pin_mut, SinkExt, StreamExt};
use shared::{FileType, TableCategory};
use crate::components::publish::edit_pending_set::upload;
use upload::Upload;
use crate::components::app::app;
use crate::components::publish::edit_pending_set::upload::UploadProps;
use crate::components::text_reader::ReadButton;


#[function_component(EditPoemList)]
pub fn edit_poem_list() -> Html {
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    if use_is_first_mount() {
        let auth = auth_ctx.clone();
        let poem_list_ctx = poem_list_ctx.clone();
        let msg_ctx = msg_ctx.clone();
        let user_uuid = auth.user_uuid.unwrap();
        let set_uuid = edit_set_ctx.editable_set.clone().unwrap().set_uuid;
        use_async::<_, (), String>(async move {
            match auth.poem_uuids_by_set_uuid(set_uuid).await? {
                GraphQlResp::Data(data) => {
                    for uuid in data.poem_uuids_by_set_uuid
                        .iter()
                        .map(|uuid| Uuid::from_str(&uuid).unwrap())
                        .collect::<Vec<Uuid>>() {
                        match auth.poem_query(uuid).await? {
                            GraphQlResp::Data(data) => {
                                if let Some(poem) = data.poem {
                                    poem_list_ctx.dispatch(EditPoemListAction::PushPoemData(
                                        PoemData {
                                            uuid,
                                            title: poem.title,
                                            set_uuid:Uuid::from_str(&poem.set_uuid).unwrap(),
                                            banter_uuid: poem.banter_uuid
                                                .map(|uuid|Uuid::from_str(&uuid).unwrap()),
                                            idx: poem.idx,
                                            approved: poem.approved
                                        }));
                                } else {
                                    msg_ctx.dispatch(
                                        new_red_msg_with_std_duration(
                                            "Can't find poem.".into()
                                        )
                                    );
                                }
                            },
                            GraphQlResp::Err(errors) => {
                                msg_ctx.dispatch(errors.into_msg_action());
                            }
                        }
                    };
                },
                GraphQlResp::Err(errors) => {
                    msg_ctx.dispatch(errors.into_msg_action());
                }}
            Ok(())
        }).run();
    }
    return html!{
        <div>
        <AddPoem/>
        <br/>
        <PoemList/>
        </div>
    };
}


#[function_component(PoemList)]
pub fn poem_list() -> Html {
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let sorted_poem_html: Html = poem_list_ctx
        .sorted_poem_data()
        .into_iter()
        .map(|data|
            html!{<Poem key={data.uuid.as_u128()} ..data.clone().into()/>})
        .collect();
    html!{
        <div>
        {sorted_poem_html}
        </div>
    }
}





