use anyhow::Result;
use graphql_client::GraphQLQuery;
use std::sync::Arc;

pub async fn post_graphql<Q: GraphQLQuery>(
    vars: <Q as GraphQLQuery>::Variables,
    jwt: Option<String>,
) -> Result<Arc<graphql_client::Response<Q::ResponseData>>> {
    let req = gloo::net::http::Request::post("api/graphql");
    let req = if jwt.is_some() {
        req.header("x-authorization", &(jwt.unwrap_or(String::default())))
    } else {
        req
    };
    //Turns our variables into a GraphQL query JSON formatted string.
    // We need an Arc here because we want to call it from use_async,
    // response is not clone and use_async's future state require clones? (I think, not sure)
    Ok(Arc::new(
        req.json(&Q::build_query(vars))?
            .send()
            .await?
            .json()
            .await?,
    ))
}
#[derive(PartialEq, Clone)]
pub enum GraphQlResp<ResponseData> {
    Data(ResponseData),
    Err(Vec<graphql_client::Error>),
    ErrExpectingData,
}
use crate::queries::{invite_user_mutation, InviteUserMutation};
#[cfg(test)]
use wasm_bindgen_test::*;

pub async fn invite_poet(
    email: String,
    token: Option<String>,
) -> Result<GraphQlResp<invite_user_mutation::ResponseData>, String> {
    // Get the values from the fields and post a login graphql query to our server
    let resp = post_graphql::<InviteUserMutation>(
        invite_user_mutation::Variables {
            email,
            user_role: invite_user_mutation::UserRole::POET,
        },
        token,
    )
    .await
    .map_err(|err| format!("{:?}", err))?;
    // If we our response has data check it's .login field it ~should~ be a jwt string
    // which we dispatch to our AuthToken which will now use it in all future contexts.

    if let Some(data) = resp.data.clone() {
        return Ok(GraphQlResp::Data(data));
    }
    // If we have no data then see if we have errors and print those to console.
    else if let Some(errors) = resp.errors.clone() {
        return Ok(GraphQlResp::Err(errors));
    }
    Ok(GraphQlResp::ErrExpectingData)
}

#[cfg_attr(test, wasm_bindgen_test)]
#[cfg_attr(test, tracing_test::traced_test)]
#[cfg(test)]
async fn test_invite_poet() {
    wasm_bindgen_test_configure!(run_in_browser);

    let resp = invite_poet("test_email@test_email.test_email".into(), None)
        .await
        .unwrap();
    match resp {
        GraphQlResp::Data(_) => {}
        GraphQlResp::Err(errors) => no_graphql_errors_or_print_them(errors).unwrap(),
        GraphQlResp::ErrExpectingData => panic!(),
    }
}
pub(crate) fn no_graphql_errors_or_print_them(
    errors: Vec<graphql_client::Error>,
) -> Result<(), ()> {
    if !errors.is_empty() {
        tracing::error!("{:?}", errors);
    }
    if !errors.is_empty() {
        Err(())?
    }
    Ok(())
}
