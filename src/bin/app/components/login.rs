use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;
use yew::prelude::*;
use crate::services::network::post_graphql;
use crate::types::auth_context::{AuthContext, AuthTokenAction};
use crate::queries::{LoginQuery,login_query::Variables};
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
            let resp = post_graphql::<LoginQuery>(Variables {
                email:email.cast::<HtmlInputElement>().unwrap().value(),
                pass:pass.cast::<HtmlInputElement>().unwrap().value(),
            }).await
                .map_err(|err| format!("{:?}", err))?;
            // If we our response has data check it's .login field it ~should~ be a jwt string
            // which we dispatch to our AuthToken which will now use it in all future contexts.
            if let Some(ref data) = resp.data {
                auth_ctx.dispatch(AuthTokenAction::Set(data.login.clone()))
            }
            // If we have no data then see if we have errors and print those to console.
            else if resp.errors.is_some() {
                tracing::error!("{:?}",resp.errors);
            }
            Ok(())
        })
    };
    // .prevent_default() is required for custom behavior for on submit buttons on forms.
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        req.run();
    });

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        ref={email.clone()}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        ref={pass.clone()}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}