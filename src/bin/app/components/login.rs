use stylist::css;
use crate::queries::{login_mutation::Variables, LoginMutation};
use crate::services::network::post_graphql;
use crate::types::auth_context::{AuthContext, AuthTokenAction};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::styles::{form_css, form_elem};
use crate::types::footer_context::{FooterContext, FooterOptionsActions};

#[function_component(Login)]
pub fn login() -> Html {

    // We'll use these node refs in our inputs on our login form.
    let email = use_node_ref();
    let pass = use_node_ref();
    // AuthContext is a ReducerHandle wrapped around Auth, so we can mutate our authtoken.
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let req = {
        // Clones are required because of the move in our async block.
        let email = email.clone();
        let pass = pass.clone();
        let auth_ctx = auth_ctx.clone();
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
                auth_ctx.dispatch(AuthTokenAction::Set(data.login.clone()))
            }
            // If we have no data then see if we have errors and print those to console.
            else if resp.errors.is_some() {
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
