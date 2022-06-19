pub mod invite_poet;
pub mod publish_menu;
pub mod edit_pending_set;
pub mod poem_list;

use crate::components::publish::publish_menu::PublishMenu;
use crate::components::publish::invite_poet::InvitePoet;

use std::str::FromStr;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount, use_mount};
use yew_router::history::History;
use yew_router::prelude::use_history;
use crate::queries::{
    invite_user_mutation, InviteUserMutation,
    pending_set_by_user_query, PendingSetByUserQuery,
    CreatePendingSetMutation, create_pending_set_mutation,
    UpdateTitleMutation, update_title_mutation,
    UpdateLinkMutation, update_link_mutation,
    AddPoemMutation, add_poem_mutation,
    PoemUuidsBySetUuidQuery, poem_uuids_by_set_uuid_query,
};
use crate::routes::Route;
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::msg_context::{MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration};


#[function_component(Publish)]
pub fn publish() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();

    html!{
        <div>
        if auth_ctx.user_role >= UserRole::Moderator {
            <InvitePoet/>
        }
        if auth_ctx.user_role >= UserRole::Poet{
        <PublishMenu/>
        } else {
            <PublicPublishInfo/>
        }
        </div>
    }
}



#[function_component(PublicPublishInfo)]
pub fn public_publish_info() -> Html {

    html!{
        <p>{"Public publish info"}</p>
    }
}