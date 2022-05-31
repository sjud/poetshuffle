use crate::graphql::resolvers::login::LoginQuery;
use async_graphql::{extensions::Tracing, *};
use hmac::Hmac;
use sea_orm::DatabaseConnection;
use sha2::Sha256;

#[derive(MergedObject, Default)]
pub struct Query(LoginQuery);
pub type PoetShuffleSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds our Schema for our service layer using DB Conn.
/// It generates internally a JWT key by using the env var JWT_SECRET.
pub fn new_schema(conn: DatabaseConnection,key:Hmac<Sha256>) -> PoetShuffleSchema {
    // Build our schema from our merged top level queries, and add
    // a database conneciton and JWT key.
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(conn)
        .data(key)
        // Tracing extension logs query info at the INFO level.
        .extension(Tracing)
        .finish()
}
