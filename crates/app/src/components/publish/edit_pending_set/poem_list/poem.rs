use super::*;

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

