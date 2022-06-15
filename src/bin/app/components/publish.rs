use std::str::FromStr;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};
use crate::queries::{
    invite_user_mutation,
    InviteUserMutation,
    pending_set_by_user_query,
    PendingSetByUserQuery,
};
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

#[function_component(InvitePoet)]
pub fn invite_poet() -> Html {
    // We'll use these node refs in our inputs on our login form.
    let email = use_node_ref();
    // AuthContext is a ReducerHandle wrapped around Auth, so we can mutate our authtoken.
    let auth_ctx = use_context::<AuthContext>().unwrap();
    // MsgContext is used to inform user of responses.
    let msg_context = use_context::<MsgContext>().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let email = email.clone();
        let token = auth_ctx.token.clone();
        // We run this when we submit our form.
        use_async::<_, (), String>(async move {
            // Get the values from the fields and post a login graphql query to our server
            let resp = post_graphql::<InviteUserMutation>(
                invite_user_mutation::Variables {
                    email: email.cast::<HtmlInputElement>().unwrap().value(),
                    user_role:invite_user_mutation::UserRole::POET,
                },token)
                .await
                .map_err(|err| format!("{:?}", err))?;
            // If we our response has data check it's .login field it ~should~ be a jwt string
            // which we dispatch to our AuthToken which will now use it in all future contexts.

            if let Some(ref data) = resp.data {
                msg_context.dispatch(new_green_msg_with_std_duration(
                    data.invite_user.clone()));
            }
            // If we have no data then see if we have errors and print those to console.
            else if resp.errors.is_some() {
                msg_context.dispatch(new_red_msg_with_std_duration(
                    map_graphql_errors_to_string(
                        &resp.errors
                    )
                ));
                tracing::error!("{:?}", resp.errors);
            }
            Ok(())
        })
    };
    // .prevent_default() is required for custom behavior for on submit buttons on forms.
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        req.run();
    });
    let form_elem = form_elem();
    let button = crate::styles::button();
    let form_css = form_css();
    html! {
        <div class={form_css.clone()}>
        <div>
            <h3>{ "Invite Poet" }</h3>
        </div>
            <form {onsubmit}>
                <input type="email" placeholder="Email" ref={email.clone()}
        class={form_elem.clone()}/>
                <br/>
                <button type="submit" disabled=false class={button.clone()}>
        { "Add Poet" } </button>
            </form>
        </div>
    }
}
#[derive(Default,PartialEq,Clone)]
pub struct EditableSet{
    pub set_uuid: Uuid,
    pub collection_title: String,
    pub collection_link: String,
}
#[function_component(PublishMenu)]
pub fn publish_menu() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let msg_context = use_context::<MsgContext>().unwrap();
    let pending_set : UseStateHandle<Option<EditableSet>> = use_state(||None);
    // If a user uuid is available find the pending set of the user.
    if let Some(user_uuid) = auth_ctx.user_uuid {
        let req = {
            let token = auth_ctx.token.clone();
            let pending_set = pending_set.clone();
            use_async::<_, (), String>(async move {
                let resp = post_graphql::<PendingSetByUserQuery>(
                    pending_set_by_user_query::Variables {
                        user_uuid: user_uuid.to_string(),
                    }, token)
                    .await
                    .map_err(|err| format!("{:?}", err))?;
                if let Some(ref data) = resp.data {
                    if let Some(set) = &data.pending_set_by_user {
                        pending_set.set(Some(EditableSet {
                            set_uuid: Uuid::from_str(&set.set_uuid)
                                .unwrap(),
                            collection_link: set.collection_link.clone(),
                            collection_title: set.collection_title.clone(),
                        }));
                        msg_context.dispatch(
                            new_green_msg_with_std_duration(
                                "Pending Set Found, Updating...".to_string())
                        )
                    };
                }
                // If we have no data then see if we have errors and print those to console.
                else if resp.errors.is_some() {
                    msg_context.dispatch(new_red_msg_with_std_duration(
                        map_graphql_errors_to_string(
                            &resp.errors
                        )
                    ));
                    tracing::error!("{:?}", resp.errors);
                }
                Ok(())
            })
        };
        if use_is_first_mount() {
            req.run();
        }
    };
    html!{
        <div>
        <h2>{"Publish Menu"}</h2>
        if (*pending_set).clone().is_some() {
            <button>{"Edit Pending Set"}</button>
        } else {
            <button>{"Create New Set"}</button>
        }
        </div>
    }
}
#[derive(Properties,PartialEq)]
pub struct EditPendingSetProps{
    editable_set:EditableSet,
}
#[function_component(EditPendingSet)]
pub fn edit_pending_set(props:&EditPendingSetProps) -> Html {
    html!{

    }
}
#[function_component(PublicPublishInfo)]
pub fn public_publish_info() -> Html {
    html!{
        <p>{"Public publish info"}</p>
    }
}