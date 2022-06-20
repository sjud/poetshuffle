pub mod invite_poet;
pub mod publish_menu;
pub mod menu;

use menu::PublishMenu;
use crate::components::publish::invite_poet::InvitePoet;

use std::str::FromStr;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount, use_mount};
use yew_router::history::History;
use yew_router::prelude::use_history;
use crate::queries::{
    add_poem_mutation, AddPoemMutation,
    create_pending_set_mutation, CreatePendingSetMutation,
    invite_user_mutation, InviteUserMutation,
    pending_set_by_user_query, PendingSetByUserQuery,
    poem_uuids_by_set_uuid_query, PoemUuidsBySetUuidQuery,
    update_link_mutation, update_title_mutation,
    UpdateLinkMutation, UpdateTitleMutation,
};
use crate::routes::Route;
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::edit_set_context::{EditSetData,EditSetContext};
use crate::types::msg_context::{MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration};


#[function_component(Publish)]
pub fn publish() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let edit_set_context = use_reducer(||EditSetData::default());

    html!{
        <div>
        if auth_ctx.user_role >= UserRole::Moderator {
            <InvitePoet/>
        }
        if auth_ctx.user_role >= UserRole::Poet {
        <ContextProvider<EditSetContext> context={edit_set_context}>
            <PublishMenu/>
            </ContextProvider<EditSetContext>>
        } else {<PublicPublishInfo/>}
        </div>
    }
}



#[function_component(PublicPublishInfo)]
pub fn public_publish_info() -> Html {

    html!{
        <p>{"Public publish info"}</p>
    }
}