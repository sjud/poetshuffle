use gloo::net::websocket::Message;
use js_sys::Uint8Array;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::HtmlSelectElement;
use crate::components::audio::{PlayButtonProps,PlayButton};
use crate::services::network::{GraphQlResp, XCategory, XFileType};
use crate::types::edit_poem_list_context::{EditPoemListAction, EditPoemListContext, PoemData};
use crate::components::publish::*;
use futures::{pin_mut, SinkExt, StreamExt};
use shared::{FileType, TableCategory};
use crate::components::publish::edit_pending_set::upload;
use upload::Upload;
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
            match auth.poem_uuids_by_set_uuid(
                Uuid::from_str(&set_uuid.to_string()).unwrap()).await? {
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
                                            idx: poem.idx
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

#[function_component(AddPoem)]
pub fn add_poem() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_set_context = use_context::<EditSetContext>().unwrap();
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let title_ref = use_node_ref();
    let add_poem = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let poem_list_ctx = poem_list_ctx.clone();
        let editable_set = edit_set_context.editable_set.clone().unwrap();
        use_async::<_, (), String>(async move {
            let set_uuid = editable_set.set_uuid;
            match auth.add_poem(set_uuid)
                .await? {
                GraphQlResp::Data(data) => {
                    poem_list_ctx.dispatch(
                        EditPoemListAction::PushPoemData(
                            PoemData{
                                uuid: Uuid::from_str(&data.add_poem.poem_uuid).unwrap(),
                                title: String::new(),
                                idx: data.add_poem.idx,
                                banter_uuid: None,
                                set_uuid,
                            }
                        ));
                    msg_context.dispatch(new_green_msg_with_std_duration("Poem Added".into()));
                },
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let add_poem = Callback::from(move |_| {
        add_poem.run();
    });
    return html! {
        <div>
        <h2>{"Add Poem to Set"}</h2>
            <button onclick={add_poem.clone()}>{"Add Poem"}</button>
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

#[derive(PartialEq, Properties, Clone,Copy,Debug)]
pub struct PoemProps{
    pub(crate) uuid:Uuid,
}

impl From<PoemData> for PoemProps {
    fn from(PoemData{uuid,..}: PoemData) -> Self { Self{ uuid } }
}

#[function_component(Poem)]
pub fn poem(props:&PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let poem = edit_poem_list_ctx.find_by_poem_uuid(props.uuid).unwrap();

    html!{
        <div>
        <h3>{poem.title.clone()}</h3>
        <UpdatePoemTitle ..*props/>
        <UpdatePoemIdx ..*props/>
        <UploadPoemAudio ..*props/>
        <UploadPoemTranscript ..*props/>
        <DeletePoem ..*props/>
        {
            if auth_ctx.user_role >= UserRole::Moderator {
            html!{<ApprovePoem ..*props/>}
        } else {
            html!{}
            }
        }
        <Banter ..*props/>

        </div>
    }
}

#[function_component(DeletePoem)]
pub fn delete_poem(props:&PoemProps) -> Html {
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let check_ref = use_node_ref();
    let delete = {
        let auth = auth_ctx.clone();
        let check_ref = check_ref.clone();
        let poem_uuid = props.uuid;
        use_async::<_,(),String>(async move {
                match auth.update_poem(
                    poem_uuid,
                    None,
                    None,
                    Some(true),
                    None,
                ).await? {
                    GraphQlResp::Data(data) => {
                        edit_poem_list_ctx.dispatch(EditPoemListAction::DeletePoemData(
                            edit_poem_list_ctx.find_by_poem_uuid(poem_uuid).unwrap()
                        ));
                        msg_ctx.dispatch(
                            new_green_msg_with_std_duration(data.update_poem)
                        );
                    },
                    GraphQlResp::Err(errors) => {
                        msg_ctx.dispatch(errors.into_msg_action());
                    }
                }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        delete.run()
    });
    html!{
        <div>
        <button {onclick}>{"Delete Poem"}</button>
        </div>
    }
}
#[function_component(ApprovePoem)]
pub fn approve_poem(props:&PoemProps) -> Html {
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let approve = {
        let auth = auth_ctx.clone();
        let uuid = props.uuid;
        use_async::<_,(),String>(async move {
                match auth.update_poem(
                    uuid,
                    None,
                    None,
                    None,
                    Some(true),
                ).await? {
                    GraphQlResp::Data(data) => {
                        msg_ctx.dispatch(
                            new_green_msg_with_std_duration(data.update_poem)
                        );
                    },
                    GraphQlResp::Err(errors) => {
                        msg_ctx.dispatch(errors.into_msg_action());
                    }
                }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        approve.run()
    });
    html!{
        <button {onclick}>{"Approve Poem"}</button>
    }
}
#[function_component(UpdatePoemTitle)]
pub fn update_poem_title(props:&PoemProps) -> Html {
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let title_ref = use_node_ref();
    let update_title = {
        let auth = auth_ctx.clone();
        let poem_uuid = props.uuid;
        let title_ref = title_ref.clone();
        use_async::<_,(),String>(async move {
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            match auth.update_poem(
                poem_uuid,
                None,
                Some(title.clone()),
                None,
                None,
            ).await? {
                GraphQlResp::Data(data) => {
                    let poem_data = edit_poem_list_ctx
                        .find_by_poem_uuid(poem_uuid)
                        .unwrap();
                    edit_poem_list_ctx.dispatch(EditPoemListAction::UpdatePoemData(
                        PoemData{ title, ..poem_data }));
                    msg_ctx.dispatch(
                        new_green_msg_with_std_duration(data.update_poem)
                    );
                },
                GraphQlResp::Err(errors) => {
                    msg_ctx.dispatch(errors.into_msg_action());
                }
            }
            Ok(())
        })
    };
    let onclick= Callback::from(move|_|{
        update_title.run()
    });
    html!{
            <div>
            <input ref={title_ref.clone()}/>
            <button {onclick}>{"Update Title"}</button>
            </div>
    }
}
#[function_component(UpdatePoemIdx)]
pub fn update_idx(props:&PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let edit_poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let poem = edit_poem_list_ctx.find_by_poem_uuid(props.uuid).unwrap();
    let set_uuid = poem.set_uuid;
    let poem_a_idx = poem.idx;
    let list_len = edit_poem_list_ctx.poems.len();
    let select_ref = use_node_ref();
    // We display indexes as 1 greater, but store true values in value attribute.
    let select_swap_html = (1..=list_len)
        .into_iter()
        .map(|i|
            html!{<option value={(i-1).to_string()}>{i}</option>})
        .collect::<Html>();
    let swap = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let edit_poem_list_ctx = edit_poem_list_ctx.clone();
        let select_ref = select_ref.clone();
        use_async::<_, (), String>(async move {
            // We only cast value when updated.
            let poem_b_idx = select_ref.cast::<HtmlSelectElement>().unwrap().value();
            let poem_b_idx = i64::from_str(&poem_b_idx)
                .map_err(|err|format!("{:?}",err))?;
            match auth.update_poem_idx(
                set_uuid, poem_a_idx,poem_b_idx).await? {
                GraphQlResp::Data(data) => {
                    edit_poem_list_ctx.dispatch(
                        EditPoemListAction::SwapIdx(poem_a_idx,poem_b_idx));
                    msg_context.dispatch(new_green_msg_with_std_duration(data.update_poem_idx));
                }
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let swap = Callback::from(move |_| {
        swap.run();
    });
    html!{
        <div>
        <span>
        {"Order:" }{poem_a_idx+1}{" swap to : "}
        <select ref={select_ref.clone()}>{select_swap_html}</select>
        </span>
        <button onclick={swap.clone()}>{"Swap Order"}</button>
        </div>
    }
}




#[function_component(UploadPoemAudio)]
pub fn upload_poem_audio(props:&PoemProps) -> Html {
    let upload_props = upload::UploadProps{
        file_ty: FileType::Audio,
        tab_cat: TableCategory::Poems,
        upload_msg: "Upload Poem Audio".to_string(),
        uuid: props.uuid
    };
    let play_btn_props = PlayButtonProps{
        uuid: props.uuid,
        tab_cat:TableCategory::Poems
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <PlayButton ..play_btn_props/>
        </div>
    }
}

#[function_component(UploadPoemTranscript)]
pub fn upload_poem_transcript(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        file_ty: FileType::Transcript,
        tab_cat: TableCategory::Poems,
        upload_msg: "Upload Poem Transcript".to_string(),
        uuid: props.uuid
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <ReadButton/>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone,Copy,Debug)]
pub struct BanterProps{
    poem_props:PoemProps,
    banter_uuid:Option<Uuid>,
}
#[function_component(Banter)]
pub fn banter(poem_props:&PoemProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let edit_poem_ctx = use_context::<EditPoemListContext>().unwrap();
    let banter_uuid = {
        if let Some(poem_data) = edit_poem_ctx.find_by_poem_uuid(poem_props.uuid)
        { poem_data.banter_uuid } else {
            //TODO Handle unhandled program state.
            panic!("\
Banter props requires poem props which requires a PoemData inside edit_poem_ctx to exist with a shared poem_uuid.\
We've panicked because we have a poem props which we've passed to banter but could not find a PoemData\
in edit_poem_ctx given a supposedly shared poem_uuid...")
        }
    };
    let banter_props = BanterProps{
        poem_props:poem_props.clone(),
        banter_uuid
    };
    let banter_exists_html =
        html!{
                <div>
            <DeleteBanter ..banter_props/>
            <ApproveBanter ..banter_props/>
            <UploadBanterAudio ..*poem_props/>
            <UploadBanterTranscript ..*poem_props/>
        {
            if auth_ctx.user_role >= UserRole::Moderator {
            html!{<ApproveBanter ..banter_props/>}
        } else {
            html!{}
            }
        }
            </div>};
    return html!{
        <div>
        { if banter_uuid.is_some(){
            {banter_exists_html}
        } else {
            html!{
                <AddBanter ..banter_props/>
            }
        }}
        </div>
    };
}
#[function_component(AddBanter)]
pub fn add_banter(props:&BanterProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let poem_list_ctx = use_context::<EditPoemListContext>().unwrap();
    let add_banter = {
        let auth = auth_ctx.clone();
        let msg_context = msg_context.clone();
        let poem_list_ctx = poem_list_ctx.clone();
        let props = props.clone();
        use_async::<_, (), String>(async move {
            match auth.add_banter(props.poem_props.uuid)
                .await? {
                GraphQlResp::Data(data) => {
                    poem_list_ctx.dispatch(
                        EditPoemListAction::UpdatePoemWithBanter{
                            poem_uuid: props.poem_props.uuid,
                            banter_uuid: Some(
                                Uuid::from_str(
                                &data.add_banter.banter_uuid)
                                    .unwrap()),
                        });
                    msg_context.dispatch(
                        new_green_msg_with_std_duration("Banter Added".into()));
                },
                GraphQlResp::Err(errors) =>
                    msg_context.dispatch(errors.into_msg_action())
            }
            Ok(())
        })
    };
    let onclick = Callback::from(move |_| {
        add_banter.run();
    });
    return html! {
        <div>
        <h2>{"Add Poem to Set"}</h2>
            <button {onclick}>{"Add Poem"}</button>
        </div>
    };
}
#[function_component(DeleteBanter)]
pub fn delete_banter(props:&BanterProps) -> Html {
    html!{}
}
#[function_component(ApproveBanter)]
pub fn approve_banter(props:&BanterProps) -> Html {
    html!{}
}
#[function_component(UploadBanterAudio)]
pub fn upload_banter_audio(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        file_ty: FileType::Audio,
        tab_cat: TableCategory::Banters,
        upload_msg: "Upload Banter Audio".to_string(),
        uuid: props.uuid
    };
    let play_btn_props = PlayButtonProps{
        uuid: props.uuid,
        tab_cat:TableCategory::Banters
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <PlayButton ..play_btn_props/>
        </div>
    }
}
#[function_component(UploadBanterTranscript)]
pub fn upload_banter_transcript(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        file_ty: FileType::Transcript,
        tab_cat: TableCategory::Poems,
        upload_msg: "Upload Poem Transcript".to_string(),
        uuid: props.uuid
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <ReadButton/>
        </div>
    }
}