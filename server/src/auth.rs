use std::collections::BTreeMap;
use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;
use anyhow::Result;
pub fn jwt(
           key:&Hmac<Sha256>,
    permission:entity::permissions::Model) -> Result<String> {
    let mut claims = BTreeMap::new();

    // Serialize the permission as the "sub" value of our future token.
    claims.insert("sub", permission);

    // Sign our claims and return functioning JWT.
    Ok(claims.sign_with_key(key)?)
}