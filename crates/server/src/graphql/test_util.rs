use crate::email::{MockEmail, TestEmail};
use crate::DATABASE_URL;
use hmac::digest::KeyInit;
use hmac::Hmac;
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use std::sync::{Arc, Mutex};
pub(crate) async fn key_conn_email() -> (Hmac<Sha256>, DatabaseConnection, TestEmail) {
    (
        Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
            .expect("Expecting valid Hmac<Sha256> from slice."),
        sea_orm::Database::connect(&*DATABASE_URL)
            .await
            .expect("Expecting DB connection given DATABASE_URL."),
        {
            let register_code = Arc::new(Mutex::new(String::default()));
            let reset_pass_code = Arc::new(Mutex::new(String::default()));
            let invite_uuid = Arc::new(Mutex::new(Uuid::default().into()));
            let mut email = TestEmail {
                register_code: register_code.clone(),
                reset_pass_code: reset_pass_code.clone(),
                invite_uuid: invite_uuid.clone(),
                email: MockEmail::new(),
            };
            email
                .email
                .expect_register()
                .returning(move |_, lost_password_code| {
                    let mut data = register_code.lock().unwrap();
                    *data = lost_password_code;
                    Ok(())
                });
            email
                .email
                .expect_reset_password()
                .returning(move |_, lost_password_code| {
                    let mut data = reset_pass_code.lock().unwrap();
                    *data = lost_password_code;
                    Ok(())
                });
            email.email.expect_invite_user().returning(move |_, uuid| {
                let mut data = invite_uuid.lock().unwrap();
                *data = uuid;
                Ok(())
            });
            email
        },
    )
}
pub(crate) fn no_graphql_errors_or_print_them(
    errors: Vec<async_graphql::ServerError>,
) -> Result<(), ()> {
    if !errors.is_empty() {
        eprintln!("{:?}", errors);
    }
    if !errors.is_empty() {
        Err(())?
    }
    Ok(())
}
