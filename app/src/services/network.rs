use anyhow::Result;
use graphql_client::GraphQLQuery;
use std::sync::Arc;
use uuid::Uuid;

pub async fn post_graphql<Q: GraphQLQuery>(
    vars: <Q as GraphQLQuery>::Variables,
    jwt: Option<String>,
) -> Result<Arc<graphql_client::Response<Q::ResponseData>>> {
    tracing::error!("hi");
    let req = gloo::net::http::Request::post(&format!("{}api/graphql",BASE_URL));
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
            .await.unwrap()
            .json()
            .await.unwrap(),
    ))
}
#[derive(PartialEq, Clone,Debug)]
pub enum GraphQlResp<ResponseData> {
    Data(ResponseData),
    Err(GraphQlRespErrors),
}
#[derive(PartialEq, Clone,Debug)]
pub struct GraphQlRespErrors(pub Option<Vec<graphql_client::Error>>);
impl GraphQlRespErrors {
    pub fn into_msg_action(self) -> MsgActions {
        new_red_msg_with_std_duration(
            map_graphql_errors_to_string(
                &self.0,
            ))
    }
}
use crate::queries::*;
#[cfg(test)]
use wasm_bindgen_test::*;
use crate::BASE_URL;
use crate::services::utility::map_graphql_errors_to_string;
use crate::types::auth_context::{AuthContext, AuthToken, UserRole};
use crate::types::msg_context::{MsgActions, new_red_msg_with_std_duration};

pub type GraphQlResult<ResponseData> = Result<GraphQlResp<ResponseData>,String>;

pub fn parse_graph_ql_resp<Data:Clone>(resp:Result<Arc<graphql_client::Response<Data>>>)
    -> Result<GraphQlResp<Data>,String> {
    let resp = resp
        .map_err(|err| format!("{:?}", err))?;
    return if let Some(data) = resp.as_ref().data.clone() {
        Ok(GraphQlResp::Data(data))
    } else {
        tracing::error!("{:?}",resp.errors);
        Ok(GraphQlResp::Err(GraphQlRespErrors(resp.as_ref().errors.clone())))
    }
}

impl AuthToken {
    #[cfg(test)]
    pub async fn new_from_login_super_admin() -> Self {
        match parse_graph_ql_resp(post_graphql::<SuperAdminLoginMutation>(
            super_admin_login_mutation::Variables {
                email: crate::services::test_util::ADMIN_USER.to_string(),
                pass: crate::services::test_util::ADMIN_PASS.to_string(),},
            None,
        )
            .await).expect("resp from super admin login") {
            GraphQlResp::Data(data) => Self::new_from_token(data.super_admin_login)
                .unwrap(),
            GraphQlResp::Err(errs) => {
                panic!("Expecting login graphql to resolve")
            }
        }
    }
    pub async fn admin_super_login(&self,email:String,pass:String)
        -> GraphQlResult<super_admin_login_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<SuperAdminLoginMutation>(
            super_admin_login_mutation::Variables { email, pass },
            self.token.clone(),
        )
            .await)
    }
    pub async fn add_poem(&self,set_uuid:Uuid,idx:i64)
        -> GraphQlResult<add_poem_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<AddPoemMutation>(
            add_poem_mutation::Variables {
                set_uuid: set_uuid.to_string(),
                idx,
            },
            self.token.clone(),
        ).await)
    }
    pub async fn invite_poet(&self, email: String)
                             -> GraphQlResult<invite_user_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<InviteUserMutation>(
            invite_user_mutation::Variables {
                email,
                user_role: invite_user_mutation::UserRole::POET,
            },
            self.token.clone(),
        ).await)
    }
    pub async fn poem_query(&self, uuid:Uuid)
                            -> GraphQlResult<poem_query::ResponseData> {
        parse_graph_ql_resp(post_graphql::<PoemQuery>(
            poem_query::Variables {
                poem_uuid: uuid.to_string(),
            },
            self.token.clone(),
        ).await)
    }
    pub async fn update_link(&self,set_uuid:Uuid,link:String)
                             -> GraphQlResult<update_link_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<UpdateLinkMutation>(
            update_link_mutation::Variables {
                set_uuid: set_uuid.to_string(),
                link,
            },
            self.token.clone(),
        ).await)
    }
}




#[cfg_attr(test,wasm_bindgen_test(async))]
#[cfg(test)]
async fn test_invite_poet() {
    wasm_bindgen_test_configure!(run_in_browser);
    let auth =  AuthToken::default();
    let resp = auth.invite_poet("test_email@test_email.test_email".into())
        .await;
    match resp.unwrap() {
        GraphQlResp::Data(_) => {}
        GraphQlResp::Err(errors) =>
            assert_eq!("Unauthorized.",errors.0.unwrap()[0].message),
    }
    let auth = AuthToken::new_from_login_super_admin().await;
    let resp = auth.invite_poet("test_email@test_email.test_email".into())
        .await
        .unwrap();
    match resp {
        GraphQlResp::Data(_) => {},
        GraphQlResp::Err(ref errors) =>
            no_graphql_errors_or_print_them(errors.0.clone().unwrap()).unwrap()
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
