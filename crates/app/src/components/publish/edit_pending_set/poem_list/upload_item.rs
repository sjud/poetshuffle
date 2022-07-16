use super::*;
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

#[function_component(UploadBanterAudio)]
pub fn upload_banter_audio(props:&BanterProps) -> Html {
    let upload_props = UploadProps{
        file_ty: FileType::Audio,
        tab_cat: TableCategory::Banters,
        upload_msg: "Upload Banter Audio".to_string(),
        uuid: props.banter_uuid.unwrap()
    };
    let play_btn_props = PlayButtonProps{
        uuid: props.banter_uuid.unwrap(),
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
pub fn upload_banter_transcript(props:&BanterProps) -> Html {
    let upload_props = UploadProps{
        file_ty: FileType::Transcript,
        tab_cat: TableCategory::Banters,
        upload_msg: "Upload Banter Transcript".to_string(),
        uuid: props.banter_uuid.unwrap()
    };
    html!{
        <div>
        <Upload ..upload_props/>
        <ReadButton/>
        </div>
    }
}
