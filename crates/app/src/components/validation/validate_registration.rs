use super::*;
use crate::components::validation::login::{Login, LoginProps};
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::msg_context::{
    new_green_msg_with_std_duration, new_red_msg_with_std_duration, MsgContext,
};
use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ValidateRegistrationProps {
    pub(crate) code: String,
    pub(crate) email: String,
}
#[function_component(ValidateRegistration)]
pub fn validate_registration(props: &ValidateRegistrationProps) -> Html {
    let pass = use_node_ref();
    let msg_context = use_context::<MsgContext>().unwrap();
    let display_login = use_state(|| false);
    let req = {
        // Clones are required because of the move in our async block.
        let pass = pass.clone();
        let email = props.email.clone();
        let lost_password_code = props.code.clone();
        let display_login = display_login.clone();
        let msg_context = msg_context.clone();
        // We run this when we submit our form.
        use_async::<_, (), String>(async move {
            // Get the values from the fields and post a login graphql query to our server
            let resp = post_graphql::<ValidateRegistrationMutation>(
                validate_registration_mutation::Variables {
                    email,
                    new_password: pass.cast::<HtmlInputElement>().unwrap().value(),
                    lost_password_code,
                },
                None,
            )
            .await
            .map_err(|err| format!("{:?}", err))?;
            // If we our response has data check it's .login field it ~should~ be a jwt string
            // which we dispatch to our AuthToken which will now use it in all future contexts.
            if let Some(ref data) = resp.data {
                display_login.set(true);
                msg_context.dispatch(new_green_msg_with_std_duration(data.validate_user.clone()));
            }
            // If we have no data then see if we have errors and print those to console.
            else if resp.errors.is_some() {
                msg_context.dispatch(new_red_msg_with_std_duration(map_graphql_errors_to_string(
                    &resp.errors,
                )));
                tracing::error!("{:?}", resp.errors);
            }
            Ok(())
        })
    };
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        req.run();
    });
    let form_elem = form_elem();
    let button = crate::styles::button();
    let form_css = form_css();
    let login_props = LoginProps {
        super_admin_login: false,
    };
    html! {
        if !*(display_login.clone()){
    <div class={form_css.clone()}>
        <div>
        <h2>{ "Enter a Password" }</h2>
        </div>
        <form {onsubmit}>
        <input type="password" placeholder="Password" ref={pass.clone()}
    class={form_elem.clone()}/>
        <br/>
        <button type="submit" disabled=false class={button.clone()}>
        { "Complete Registration" } </button>
        </form>
        </div>
            } else {
            <Login ..login_props/>
        }
        }
}
