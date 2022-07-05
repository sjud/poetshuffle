pub mod invite_poet;
pub mod edit_pending_set;
pub mod poem_list;
pub mod edit_poem;

use crate::components::publish::invite_poet::InvitePoet;
use edit_pending_set::EditPendingSet;
use crate::queries::{
    add_poem_mutation, AddPoemMutation, create_pending_set_mutation,
    CreatePendingSetMutation, invite_user_mutation, InviteUserMutation,
    pending_set_by_user_query, PendingSetByUserQuery, poem_uuids_by_set_uuid_query, PoemUuidsBySetUuidQuery,
    update_set_mutation, UpdateSetMutation
};
use crate::routes::Route;
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem, main_menu_style};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::edit_set_context::{EditSetContext, EditSetData, EditSetActions};
use crate::types::msg_context::{
    MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration,
};
use std::str::FromStr;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount, use_mount};
use yew_router::history::History;
use yew_router::prelude::use_history;

#[function_component(Publish)]
pub fn publish() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let edit_set_context = use_reducer(|| EditSetData::default());
    html! {
        <div>
        if auth_ctx.user_role >= UserRole::Moderator {
            <InvitePoet/>
        }
        if auth_ctx.user_role >= UserRole::Poet {
        <ContextProvider<EditSetContext> context={edit_set_context}>
            <EditPendingSet/>
            </ContextProvider<EditSetContext>>
        } else {<PublicPublishInfo/>}
        </div>
    }
}

#[function_component(PublicPublishInfo)]
pub fn public_publish_info() -> Html {
    let info = use_state(||"Loading info...");
    let css = main_menu_style();
    if use_is_first_mount() {
        let info = info.clone();
        use_async::<_,(),String>(async move {

        }).run();
    };
    html! {
        <p class={css}>{(*info).clone()}</p>
    }
}
