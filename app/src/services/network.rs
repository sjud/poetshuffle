use anyhow::Result;
use graphql_client::GraphQLQuery;
use std::sync::Arc;
use uuid::Uuid;
use gloo::console::error;
use reqwest::header::HeaderValue;
use reqwest::Url;

pub async fn post_graphql<Q: GraphQLQuery>(
    vars: <Q as GraphQLQuery>::Variables,
    jwt: Option<String>,
) -> Result<Arc<graphql_client::Response<Q::ResponseData>>> {
    let req = reqwest::Client::new().post(
        &format!("{}api/graphql",BASE_URL));
    let req = if jwt.is_some() {
        req.header(
            "x-authorization",
            HeaderValue::from_str(&jwt.unwrap_or(String::default()))?
        )
    } else {
        req
    };
    Ok(Arc::new(
        req.json(&Q::build_query(vars))
            .send()
            .await?
            .json()
            .await?
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
        Ok(GraphQlResp::Err(GraphQlRespErrors(resp.as_ref().errors.clone())))
    }
}

impl AuthToken {
    pub async fn fetch_text_file(&self,path:String) -> String {
        "".into()
    }
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
                panic!("{:?}",errs)
            }
        }
    }
    pub async fn update_poem_idx(&self,set_uuid:Uuid,poem_a_idx:i64,poem_b_idx:i64)
    -> GraphQlResult<update_poem_idx_mutation::ResponseData> {
        parse_graph_ql_resp(
            post_graphql::<UpdatePoemIdxMutation>(
                update_poem_idx_mutation::Variables {
                    set_uuid: set_uuid.to_string(),
                    poem_a_idx,
                    poem_b_idx,
                },
                self.token.clone(),
            )
                .await
        )
    }
    pub async fn update_poem(&self,
                             poem_uuid:Uuid,
                             banter_uuid: Option<Uuid>,
                             title: Option<String>,
                             delete: Option<bool>,
                             approve: Option<bool>,)
        -> GraphQlResult<update_poem_mutation::ResponseData> {
        parse_graph_ql_resp(
            post_graphql::<UpdatePoemMutation>(
                update_poem_mutation::Variables {
                    poem_uuid: poem_uuid.to_string(),
                    banter_uuid: banter_uuid.map(|uuid|uuid.to_string()),
                    title, delete,approve
                },
                self.token.clone(),
            )
                .await
        )
    }
    pub async fn update_set(&self,
                            set_uuid:Uuid,
                            title:Option<String>,
                            link:Option<String>,
                            delete:Option<bool>,
                            approve:Option<bool>,)
    -> GraphQlResult<update_set_mutation::ResponseData> {
        parse_graph_ql_resp(
            post_graphql::<UpdateSetMutation>(
                update_set_mutation::Variables {
                    set_uuid: set_uuid.to_string(),
                    title,link,delete,approve
                },
                self.token.clone(),
            )
                .await
        )
    }
    pub async fn pending_set_by_user__uuid(&self,user_uuid:Uuid)
        -> GraphQlResult<pending_set_by_user_query::ResponseData> {
        parse_graph_ql_resp( post_graphql::<PendingSetByUserQuery>(
            pending_set_by_user_query::Variables { user_uuid: user_uuid.to_string()},
            self.token.clone(),
        )
            .await)
    }
    pub async fn poem_uuids_by_set_uuid(&self,set_uuid:Uuid)
        -> GraphQlResult<poem_uuids_by_set_uuid_query::ResponseData> {
        parse_graph_ql_resp(
        post_graphql::<PoemUuidsBySetUuidQuery>(
            poem_uuids_by_set_uuid_query::Variables { set_uuid: set_uuid.to_string(), },
            self.token.clone(),
        )
            .await)
    }
    pub async fn admin_super_login(&self,email:String,pass:String)
        -> GraphQlResult<super_admin_login_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<SuperAdminLoginMutation>(
            super_admin_login_mutation::Variables { email, pass },
            self.token.clone(),
        )
            .await)
    }
    pub async fn add_poem(&self,set_uuid:Uuid)
        -> GraphQlResult<add_poem_mutation::ResponseData> {
        parse_graph_ql_resp(post_graphql::<AddPoemMutation>(
            add_poem_mutation::Variables {
                set_uuid: set_uuid.to_string(),
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
}


#[cfg(test)]
pub mod test {
    // Required for testing in browser.
    wasm_bindgen_test_configure!(run_in_browser);
    use reqwest::{Client, StatusCode};
    use crate::services::test_util::{ADMIN_PASS, ADMIN_USER};
    use super::*;
    /*
        Integration tests between app and server.
     */
    #[wasm_bindgen_test]
    async fn test_pending_set_by_user__uuid() {
        let auth = AuthToken::new_from_login_super_admin().await;
        let resp = auth
            .pending_set_by_user__uuid(auth.user_uuid.unwrap()).await.unwrap();
        // Response should return None, not graphql errors.
        match resp {
            GraphQlResp::Data(_) => {},
            GraphQlResp::Err(errors) =>
                no_graphql_errors_or_print_them(errors.0.clone().unwrap()).unwrap()
        }
    }
    #[wasm_bindgen_test]
    async fn test_poem_uuids_by_set_uuid() {
        let auth = AuthToken::new_from_login_super_admin().await;
        let resp = auth
            .poem_uuids_by_set_uuid(Uuid::new_v4()).await.unwrap();
        // Resolve should resolve and tell us set doesn't exist.
        match resp {
            GraphQlResp::Data(_) => {},
            GraphQlResp::Err(errors) =>
                assert_eq!("Set not found", errors.0.unwrap()[0].message),
        }

    }
    #[wasm_bindgen_test]
    async fn test_admin_super_login() {
        let auth = AuthToken::default();
        let resp = auth
            .admin_super_login(ADMIN_USER.into(),ADMIN_PASS.into()).await.unwrap();
        match resp {
            GraphQlResp::Data(_) => {},
            GraphQlResp::Err(ref errors) =>
                no_graphql_errors_or_print_them(errors.0.clone().unwrap()).unwrap()
        }
    }
    #[wasm_bindgen_test]
    async fn test_add_poem() {
        let auth = AuthToken::new_from_login_super_admin().await;
        let resp = auth
            .add_poem(Uuid::new_v4(),0).await.unwrap();
        match resp {
            GraphQlResp::Data(_) => {},
            GraphQlResp::Err(errors) =>
                assert_eq!("Set not found", errors.0.unwrap()[0].message),

        }
    }
    #[wasm_bindgen_test]
    async fn test_poem_query() {
        let auth = AuthToken::default();
        let resp : GraphQlResp<poem_query::ResponseData> = auth
            .poem_query(Uuid::new_v4()).await.unwrap();
        match resp {
            GraphQlResp::Data(data) =>
            assert_eq!(data,poem_query::ResponseData{poem:None}),
            GraphQlResp::Err(errors) =>
                no_graphql_errors_or_print_them(
                    errors.0.clone().unwrap())
                    .unwrap()
        }
    }
    #[wasm_bindgen_test]
    async fn test_health_check() {
        let resp = Client::new()
            .get(&format!("{}api/health_check",BASE_URL))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[wasm_bindgen_test]
    async fn test_invite_poet() {
        let auth = AuthToken::default();
        let resp = auth.invite_poet("test_email@test_email.test_email".into())
            .await.unwrap();
        match resp {
            GraphQlResp::Data(_) => {}
            GraphQlResp::Err(errors) =>
                assert_eq!("Unauthorized.", errors.0.unwrap()[0].message),
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
    // Uses gloo error to print to the console during tests.
    pub(crate) fn no_graphql_errors_or_print_them(
        errors: Vec<graphql_client::Error>,
    ) -> Result<(), ()> {
        if !errors.is_empty() {
            error!(format!("{:?}", errors));
        }
        if !errors.is_empty() {
            Err(())?
        }
        Ok(())
    }
}
