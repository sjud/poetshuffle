use crate::components::login::{Login, LoginProps};
use crate::queries::{
    modify_user_role_mutation::{self, UserRole as QueryUserRole},
    ModifyUserRoleMutation,
};
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::auth_context::{AuthContext, UserRole};
use crate::types::msg_context::{
    new_green_msg_with_std_duration, new_red_msg_with_std_duration, MsgContext,
};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_hooks::use_async;

impl QueryUserRole {
    pub fn from_str(role: &str) -> Option<Self> {
        match role {
            "Admin" => Some(Self::ADMIN),
            "Listener" => Some(Self::LISTENER),
            "Moderator" => Some(Self::MODERATOR),
            "Poet" => Some(Self::POET),
            "SuperAdmin" => Some(Self::SUPERADMIN),
            _ => None,
        }
    }
}

#[function_component(Admin)]
pub fn admin() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let login_props = LoginProps {
        super_admin_login: true,
    };
    let role = &auth_ctx.user_role;
    html! {
        if *role < UserRole::Admin {
            <Login ..login_props/>
        } else {
            <ModifyUserRole/>
        }
    }
}
#[function_component(ModifyUserRole)]
pub fn modify_user_role() -> Html {
    let email = use_node_ref();
    let role = use_node_ref();
    let msg_context = use_context::<MsgContext>().unwrap();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let email = email.clone();
        let token = auth_ctx.token.clone();
        let role = role.clone();
        // We run this when we submit our form.
        use_async::<_, (), String>(async move {
            // Get the values from the fields and post a login graphql query to our server
            let resp = post_graphql::<ModifyUserRoleMutation>(
                modify_user_role_mutation::Variables {
                    email: email.cast::<HtmlInputElement>().unwrap().value(),
                    new_user_role: QueryUserRole::from_str(
                        &role.cast::<HtmlSelectElement>().unwrap().value(),
                    )
                    .unwrap(),
                },
                token,
            )
            .await
            .map_err(|err| format!("{:?}", err))?;
            // If we our response has data check it's .login field it ~should~ be a jwt string
            // which we dispatch to our AuthToken which will now use it in all future contexts.

            if let Some(ref data) = resp.data {
                msg_context.dispatch(new_green_msg_with_std_duration(
                    data.modify_user_role.clone(),
                ));
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
            <h4>{ "Modify User Role" }</h4>
        </div>
            <form {onsubmit}>
                <input type="email" placeholder="Email" ref={email.clone()}
        class={form_elem.clone()}/>
                <br/>
                <select ref={role.clone()}>
        <option value="">{"Please choose an option"}</option>
            <option value="Listener">{"Listener"}</option>
            <option value="Poet">{"Poet"}</option>
            <option value="Moderator">{"Moderator"}</option>
            <option value="Admin">{"Admin"}</option>
        </select>
        <br/>
                <button type="submit" disabled=false class={button.clone()}>
        { "Modify User Role" } </button>
            </form>
        </div>

    }
}
