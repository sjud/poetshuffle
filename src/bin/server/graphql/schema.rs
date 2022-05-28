use async_graphql::{*,extensions::Tracing};
use hmac::{digest::KeyInit,Hmac};
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use crate::graphql::resolvers::login::LoginQuery;

#[derive(MergedObject, Default)]
pub struct Query(LoginQuery);
pub type PoetShuffleSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds our Schema for our service layer using DB Conn.
/// It generates internally a JWT key by using the env var JWT_SECRET.
pub fn new_schema(conn:DatabaseConnection)
                  ->  PoetShuffleSchema {
    // Create our key for signing JWT's.
    let key: Hmac<Sha256> = Hmac::new_from_slice(crate::JWT_SECRET.as_bytes())
        .expect("Expecting valid Hmac<Sha256> from slice.");
    // Build our schema from our merged top level queries, and add
    // a database conneciton and JWT key.
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(conn)
        .data(key)
        // Tracing extension logs query info at the INFO level.
        .extension(Tracing)
        .finish()
}