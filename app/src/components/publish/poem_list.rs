use std::mem::swap;
use gloo::file::futures::read_as_bytes;
use web_sys::HtmlSelectElement;
use crate::components::app::MAX_SIZE;
use crate::components::audio::{PlayButtonProps,PlayButton};
use crate::services::network::{GraphQlResp, XCategory, XFileType};
use crate::types::edit_poem_list_context::{EditPoemListAction, EditPoemListContext, PoemData};
use super::*;

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

#[derive(Properties,PartialEq,Clone)]
pub struct UploadProps{
    x_file:XFileType,
    x_category:XCategory,
    upload_msg:String,
    uuid:Uuid,
}

#[function_component(Upload)]
pub fn upload(props:&UploadProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let input_ref = use_node_ref();
    let upload = {
        let input_ref = input_ref.clone();
        let props = props.clone();
        use_async::<_,(),String>(async move {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0){
                    let file = wasm_bindgen_futures::JsFuture::from(
                        file.array_buffer())
                        .await
                        .unwrap();
                    auth_ctx.upload_file(
                        props.x_category,
                        props.x_file,
                        file,
                        props.uuid)
                        .await
                        .unwrap();
                } else{
                    msg_context.dispatch(
                        new_red_msg_with_std_duration("file.get(0) output: None".into())
                    )
                }
            }
            Ok(())
        })
    };
    let onchange = Callback::from(move|_|upload.run());
    html!{
        <div>
        <label for={props.uuid.to_string()}>{props.upload_msg.clone()}</label><br/>
        <input type="file" id={props.uuid.to_string()} {onchange} ref={input_ref}/>
        </div>
    }
}
#[function_component(UploadPoemTranscript)]
pub fn upload_poem_transcript(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        x_file: XFileType::Transcript,
        x_category: XCategory::Poem,
        upload_msg: "Upload Poem Transcript".to_string(),
        uuid: props.uuid
    };
    html!{<Upload ..upload_props/>}
}
#[function_component(UploadPoemAudio)]
pub fn upload_poem_audio(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        x_file: XFileType::Audio,
        x_category: XCategory::Poem,
        upload_msg: "Upload Poem Audio".to_string(),
        uuid: props.uuid
    };
    let play_btn_props = PlayButtonProps{
        uuid: props.uuid,
        x_cat: XCategory::Poem,
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <PlayButton ..play_btn_props/>
        </div>
    }
}
#[function_component(Banter)]
pub fn banter(props:&PoemProps) -> Html {
    html!{}
}
#[function_component(AddBanter)]
pub fn add_banter() -> Html {
    html!{}
}
#[function_component(DeleteBanter)]
pub fn delete_banter() -> Html {
    html!{}
}
#[function_component(UploadBanterAudio)]
pub fn upload_banter_audio(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        x_file: XFileType::Audio,
        x_category: XCategory::Banter,
        upload_msg: "Upload Banter Audio".to_string(),
        uuid: props.uuid
    };
    html!{<Upload ..upload_props/>}
}
#[function_component(UploadBanterTranscript)]
pub fn upload_banter_transcript(props:&PoemProps) -> Html {
    let upload_props = UploadProps{
        x_file: XFileType::Transcript,
        x_category: XCategory::Banter,
        upload_msg: "Upload Banter Audio".to_string(),
        uuid: props.uuid
    };
    html!{<Upload ..upload_props/>}
}

