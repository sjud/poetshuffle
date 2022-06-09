use anyhow::Result;
use async_graphql::validators::email;
use postmark::reqwest::PostmarkClient;
use crate::{OUTBOUND_EMAIL, URL_BASE};
use postmark::Query;

#[cfg(test)]
#[mockall::automock]
#[async_trait::async_trait]
pub trait Email {
    async fn register(&self,email:String,lost_password_code:String) -> Result<()>;
    async fn reset_password(&self,email:String,lost_password_code:String) -> Result<()>;
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
                format!("{}validate_registration/{}",
                        &*URL_BASE,
                        lost_password_code)))
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
}