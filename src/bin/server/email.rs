use anyhow::Result;
use postmark::reqwest::PostmarkClient;
use crate::{OUTBOUND_EMAIL, URL_BASE};
use postmark::Query;
use sea_orm::prelude::Uuid;


#[cfg_attr(test,mockall::automock)]
#[async_trait::async_trait]
pub trait Email {
    async fn register(&self,email:String,lost_password_code:String) -> Result<()>;
    async fn reset_password(&self,email:String,lost_password_code:String) -> Result<()>;
    async fn invite_user(&self, email:String, invite_uuid:Uuid) -> Result<()>;
}

#[cfg(test)]
pub struct TestEmail{
    pub(crate) register_code:Arc<Mutex<String>>,
    pub(crate) reset_pass_code:Arc<Mutex<String>>,
    pub(crate) invite_uuid:Arc<Mutex<Uuid>>,
    pub(crate) email:MockEmail,
}
#[cfg(test)]
#[async_trait::async_trait]
impl Email for TestEmail{
    async fn register(&self, email: String, lost_password_code: String) -> Result<()> {
        self.email.register(email,lost_password_code).await
    }

    async fn reset_password(&self, email: String, lost_password_code: String) -> Result<()> {
        self.email.reset_password(email,lost_password_code).await
    }

    async fn invite_user(&self, email: String, invite_uuid:Uuid) -> Result<()> {
        self.email.invite_user(email,invite_uuid).await
    }
}

pub struct Postmark{
    pub(crate) client:PostmarkClient,
}
#[async_trait::async_trait]
impl Email for Postmark {
    async fn register(&self, email:String,lost_password_code:String) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Registration")
            .body(postmark::api::email::Body::Text(
                format!("{}validate_registration/{}/{}",
                        &*URL_BASE,email, lost_password_code)))
            .build();
        req.execute(&self.client).await?;
        Ok(())
    }

    async fn reset_password(&self, email: String, lost_password_code: String) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Registration")
            .body(postmark::api::email::Body::Text(
                format!("{}reset_password/{}",
                        &*URL_BASE,
                        lost_password_code)))
            .build();
        let _ = req.execute(&self.client).await?;
        Ok(())
    }

    async fn invite_user(&self, email:String, invite_uuid:Uuid) -> Result<()> {
        let req = postmark::api::email::SendEmailRequest::builder()
            .from(&*OUTBOUND_EMAIL)
            .to(&email)
            .subject("PoetShuffle Invitation")
            .body(postmark::api::email::Body::Text(
                format!("You've been invited to PoetShuffle.
                {}accept_invitation/{}",
                        &*URL_BASE,
                        invite_uuid)))
            .build();
        let _ = req.execute(&self.client).await?;
        Ok(())
    }
}