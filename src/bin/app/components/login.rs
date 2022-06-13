use crate::queries::{login_mutation::Variables, LoginMutation};
use crate::services::network::post_graphql;
use crate::types::auth_context::{AuthContext, AuthTokenAction};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_history;
use yew_router::prelude::History;
use crate::MSG_DURATION;
use crate::routes::Route;
use crate::services::utility::map_graphql_errors_to_string;
use crate::styles::{form_css, form_elem};
use crate::types::msg_context::{MsgContext, new_green_msg_with_std_duration, new_red_msg_with_std_duration};

#[function_component(Login)]
pub fn login() -> Html {

    // We'll use these node refs in our inputs on our login form.
    let email = use_node_ref();
    let pass = use_node_ref();
    // AuthContext is a ReducerHandle wrapped around Auth, so we can mutate our authtoken.
    let auth_ctx = use_context::<AuthContext>().unwrap();
    // MsgContext is used to inform user of responses.
    let msg_context = use_context::<MsgContext>().unwrap();
    let history = use_history().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let email = email.clone();
        let pass = pass.clone();
        let auth_ctx = auth_ctx.clone();
        let msg_context = msg_context.clone();
        // We run this when we submit our form.
        use_async::<_, (), String>(async move {
            // Get the values from the fields and post a login graphql query to our server
            let resp = post_graphql::<LoginMutation>(Variables {
                email: email.cast::<HtmlInputElement>().unwrap().value(),
                pass: pass.cast::<HtmlInputElement>().unwrap().value(),
            },None)
            .await
            .map_err(|err| format!("{:?}", err))?;
            // If we our response has data check it's .login field it ~should~ be a jwt string
            // which we dispatch to our AuthToken which will now use it in all future contexts.
            if let Some(ref data) = resp.data {
                // data.login is field generated by graphql client, it stores our jwt.
                auth_ctx.dispatch(AuthTokenAction::Set(Some(data.login.clone())));
                msg_context.dispatch(new_green_msg_with_std_duration(
                    "Login Successful.".into()));
                history.push(Route::MainMenu);
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
            <h2>{ "Sign In" }</h2>
        </div>
            <form {onsubmit}>
                <input type="email" placeholder="Email" ref={email.clone()}
        class={form_elem.clone()}/>
                <br/>
                <input type="password" placeholder="Password" ref={pass.clone()}
        class={form_elem.clone()}/>
                <br/>
                <button type="submit" disabled=false class={button.clone()}>
        { "Sign in" } </button>
            </form>
        </div>

    }
}
