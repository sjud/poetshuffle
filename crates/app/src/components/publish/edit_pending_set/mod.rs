mod upload;
pub use upload::*;
pub mod poem_list;
mod create_set;
pub use create_set::*;
mod update_set;
pub use update_set::*;
mod delete_set;
pub use delete_set::*;

use crate::services::network::GraphQlResp;
use super::*;
use crate::types::edit_set_context::{EditableSet, EditSetActions, EditSetContext};
use crate::types::edit_poem_list_context::{EditPoemListContext, EditPoemListData};
use poem_list::EditPoemList;
use crate::queries::set::*;

#[function_component(EditPendingSet)]
pub fn edit_pending_set() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_ctx = use_context::<MsgContext>().unwrap();
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    if use_is_first_mount() {
        let auth = auth_ctx.clone();
        let edit_set_ctx = edit_set_ctx.clone();
        let msg_ctx = msg_ctx.clone();
        let user_uuid = auth.user_uuid.unwrap();
        use_async::<_, (), String>(async move {
            match auth.pending_set_by_user_uuid(user_uuid).await? {
                GraphQlResp::Data(data) => {
                    if let Some(set) = data.pending_set_by_user {
                        edit_set_ctx.dispatch(
                            EditSetActions::EditableSet(
                                Some(EditableSet {
                                    set_uuid: Uuid::from_str(&set.set_uuid).unwrap(),
                                    link: set.link.clone(),
                                    title: set.title.clone(),
                                })));
                    } else {}
                },
                GraphQlResp::Err(errors) =>
                    msg_ctx.dispatch(errors.into_msg_action()),
            }
            Ok(())}
        ).run();
    }
    return html!{<EditPendingSetDecider/>};
}
#[function_component(EditPendingSetDecider)]
pub fn edit_pending_set_decider() -> Html {
    let edit_set_ctx = use_context::<EditSetContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    if edit_set_ctx.editable_set.is_some() {
        let edit_poem_list_ctx = use_reducer(||EditPoemListData::default());
        return html! {
            <ContextProvider<EditPoemListContext> context={edit_poem_list_ctx}>
            <div>
            <h2>{"Edit Pending Set"}</h2>
            <UpdateSetTitle/>
            <br/>
            <UpdateSetLink/>
            <br/>
            <DeleteSet/>
            <br/>
            <EditPoemList/>
            </div>
            </ContextProvider<EditPoemListContext>>
    }; } else {
        return html! { <CreateSet/>};
    }
}




