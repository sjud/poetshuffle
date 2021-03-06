use anyhow::Result;
use postmark::reqwest::PostmarkClient;
use postmark::Query;
use sea_orm::prelude::Uuid;

#[cfg(feature = "mock_email")]
use std::sync::{Arc, Mutex};

lazy_static::lazy_static!{
    pub static ref POSTMARK_API_TRANSACTION: String = {
        if let Ok(api_key) = std::env::var("POSTMARK_API_TRANSACTION") {
            api_key
        } else {
             #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("POSTMARK_API_TRANSACTION").to_string();
            panic!("Requires POSTMARK_API_TRANSACTION, not set in .env or environment");
        }
    };
    pub static ref OUTBOUND_EMAIL: String = {
        if let Ok(outbound_email) = std::env::var("OUTBOUND_EMAIL") {
            outbound_email
        } else {
             #[cfg(feature = "dev")]
            return dotenv_codegen::dotenv!("OUTBOUND_EMAIL").to_string();
            panic!("Requires OUTBOUND_EMAIL, not set in .env or environment");
        }
    };
        /// i.e https://127.0.0.1:8000/
    pub static ref URL_BASE: String = {
        use crate::http::{SERVER_IP,SERVER_PORT};
        if let Ok(origin) = std::env::var("URL_BASE") {
            origin
        } else {
            return format!("http://{}:{}/",&*SERVER_IP,&*SERVER_PORT);
        }
    };
}

#[cfg_attr(feature="mock_email", mockall::automock)]
#[async_trait::async_trait]
pub trait Email {
    async fn register(&self, email: String, lost_password_code: String) -> Result<()>;
    async fn reset_password(&self, email: String, lost_password_code: String) -> Result<()>;
    async fn invite_user(&self, email: String, invite_uuid: Uuid) -> Result<()>;
}

#[cfg(feature = "mock_email")]
pub struct TestEmail {
    pub(crate) register_code: Arc<Mutex<String>>,
    pub(crate) reset_pass_code: Arc<Mutex<String>>,
    pub(crate) invite_uuid: Arc<Mutex<Uuid>>,
    pub(crate) email: MockEmail,
}
#[cfg(feature = "mock_email")]
#[async_trait::async_trait]
impl Email for TestEmail {
    async fn register(&self, email: String, lost_password_code: String) -> Result<()> {
        self.email.register(email, lost_password_code).await
    }

    async fn reset_password(&self, email: String, lost_password_code: String) -> Result<()> {
        self.email.reset_password(email, lost_password_code).await
    }

    async fn invite_user(&self, email: String, invite_uuid: Uuid) -> Result<()> {
        self.email.invite_user(email, invite_uuid).await
    }
}

pub struct Postmark {
    pub(crate) client: PostmarkClient,
}
#[async_trait::async_trait]
impl Email for Postmark {
    async fn register(&self, email: String, lost_password_code: String) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Registration")
            .body(postmark::api::email::Body::Text(format!(
                "{}validate_registration/{}/{}",
                &*URL_BASE, email, lost_password_code
            )))
            .build();
        req.execute(&self.client).await?;
        Ok(())
    }

    async fn reset_password(&self, email: String, lost_password_code: String) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Registration")
            .body(postmark::api::email::Body::Text(format!(
                "{}reset_password/{}",
                &*URL_BASE, lost_password_code
            )))
            .build();
        let _ = req.execute(&self.client).await?;
        Ok(())
    }

    async fn invite_user(&self, email: String, invite_uuid: Uuid) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Invitation")
            .body(postmark::api::email::Body::Text(format!(
                "You've been invited to PoetShuffle.
                {}accept_invitation/{}",
                &*URL_BASE, invite_uuid
            )))
            .build();
        let _ = req.execute(&self.client).await?;
        Ok(())
    }
}
