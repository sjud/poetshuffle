use std::sync::Arc;
use super::*;
use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;
use graphql_client::{GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "app_queries/login.graphql",
response_derives = "Serialize,PartialEq",
)]
struct LoginQuery;

#[function_component(Admin)]
pub fn admin() -> Html {
    html!{
        <div>
        <Login/>
        </div>
    }
}


pub async fn post_graphql<Q:GraphQLQuery>(vars:<Q as GraphQLQuery>::Variables)
    -> Result<Arc<graphql_client::Response<Q::ResponseData>>,String>{
    Ok(Arc::new(
        gloo::net::http::Request::post("api/graphql")
        .header("accept-encoding","gzip")
        .json(&Q::build_query(vars))
                 .map_err(|err|format!("{:?}",err))?
        .send()
        .await
            .map_err(|err|format!("{:?}",err))?
        .json()
        .await
            .map_err(|err|format!("{:?}",err))?
    ))
}



#[function_component(Login)]
pub fn login() -> Html {
    let email = use_node_ref();
    let pass = use_node_ref();
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let req = {
        let email = email.clone();
        let pass = pass.clone();
        let auth_ctx = auth_ctx.clone();
        use_async::<_, (), String>(async move {
            let resp = post_graphql::<LoginQuery>(login_query::Variables {
                email:email.cast::<HtmlInputElement>().unwrap().value(),
                pass:pass.cast::<HtmlInputElement>().unwrap().value(),
            }).await
                .map_err(|err| format!("{:?}", err))?;
            if let Some(ref data) = resp.data {
                auth_ctx.dispatch(AuthTokenAction::Set(data.login.clone()))
            }
            Ok(())
        }
        )
    };
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