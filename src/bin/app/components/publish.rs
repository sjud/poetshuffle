use yew::prelude::*;
use yew_hooks::use_async;
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::msg_context::{MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration};


#[function_component(Publish)]
pub fn publish() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();

    html!{
        if auth_ctx.user_role >= UserRole::Moderator {
            <InvitePoet/>
        }
        <PublishMenu/>
    }
}

#[function_component(InvitePoet)]
pub fn invite_poet() -> Html {
    html!{

    }
}
#[function_component(PublishMenu)]
pub fn publish_menu() -> Html {
    html!{
        <h2>{"Publish Menu"}</h2>
    }
}