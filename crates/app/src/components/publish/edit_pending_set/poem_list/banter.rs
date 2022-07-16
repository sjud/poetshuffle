use super::*;
#[derive(PartialEq, Properties, Clone,Copy,Debug)]
pub struct BanterProps{
    pub poem_props:PoemProps,
    pub banter_uuid:Option<Uuid>,
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
            <UploadBanterAudio ..banter_props/>
            <UploadBanterTranscript ..banter_props/>
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