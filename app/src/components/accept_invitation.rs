use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};
use crate::queries::{AcceptInvitationMutation,accept_invitation_mutation};
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::msg_context::{green_msg, MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration};

#[derive(Properties,PartialEq)]
pub struct AcceptInvitationProps{
    pub(crate) invite_uuid:Uuid,
}
#[function_component(AcceptInvitation)]
pub fn accept_invitation(props:&AcceptInvitationProps) -> Html {
    let needs_password = use_state_eq(||false);
    let msg_context = use_context::<MsgContext>().unwrap();
    let message = use_state_eq(||String::from("We're processing your invitation."));
    let req = {
        let needs_password = needs_password.clone();
        let message = message.clone();
        let invite_uuid = props.invite_uuid;
        use_async::<_, (), String>(async move {
            let resp = post_graphql::<AcceptInvitationMutation>(
                accept_invitation_mutation::Variables {
                    password: None,
                    invite_uuid: invite_uuid.to_string()
                },None
            ).await.map_err(|err| format!("{:?}", err))?;

            if let Some(ref data) = resp.data {
                if data.accept_invitation.eq("NEEDS_PASSWORD"){
                    needs_password.set(true);
                } else {
                    message.set(
                        String::from(
                            data
                                .accept_invitation
                                .clone()
                        ));
                }

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

    let password_props = PasswordProps{
        invite_uuid:props.invite_uuid,
    };
    html!{
                    <div>

                <h2>{"You've been invited to PoetShuffle\n "}</h2>
        if !*needs_password.clone(){
                    <h2>{(*message).clone()}</h2>
            } else {
            <PasswordInput ..password_props/>
        }
                    </div>


    }
}
#[derive(Properties,PartialEq)]
pub struct PasswordProps{
    pub(crate) invite_uuid:Uuid,
}
#[function_component(PasswordInput)]
pub fn password_input(props:&PasswordProps) -> Html {
    let pass = use_node_ref();
    let msg_context = use_context::<MsgContext>().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let pass = pass.clone();
        let invite_uuid = props.invite_uuid;
        use_async::<_, (), String>(async move {
            let pass = pass.cast::<HtmlInputElement>().unwrap().value();
            let resp = post_graphql::<AcceptInvitationMutation>(
                accept_invitation_mutation::Variables{
                    password:Some(pass),
                    invite_uuid:invite_uuid.to_string()
                },None
            ).await.map_err(|err| format!("{:?}", err))?;

            if let Some(ref data) = resp.data {
                msg_context.dispatch(
                    green_msg(data.accept_invitation.clone()));
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
            <h2>{ "A password is needed to create your account" }</h2>
            <form {onsubmit}>
                <input type="password" placeholder="Password" ref={pass.clone()}
        class={form_elem.clone()}/>
                <br/>
                <button type="submit" disabled=false class={button.clone()}>
        <h3>{ "Create Account" }</h3> </button>
            </form>
        </div>
    }
}