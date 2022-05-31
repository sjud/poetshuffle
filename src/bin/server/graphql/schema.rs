use crate::graphql::resolvers::login::{LoginMutation};
use postmark::reqwest::PostmarkClient;
use async_graphql::{extensions::Tracing, *};
use hmac::Hmac;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use crate::POSTMARK_API_TRANSACTION;

#[derive(MergedObject, Default)]
pub struct Query();
#[derive(MergedObject, Default)]
pub struct Mutation(LoginMutation);
pub type PoetShuffleSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds our Schema for our service layer using DB Conn.
/// It generates internally a JWT key by using the env var JWT_SECRET.
pub fn new_schema(conn: DatabaseConnection,key:Hmac<Sha256>) -> PoetShuffleSchema {
    // Build our email client for our schema to send emails.
    let client = PostmarkClient::builder()
        .base_url("https://api.postmarkapp.com/")
        .token(&*POSTMARK_API_TRANSACTION)
        .build();
    // Build our schema from our merged top level queries, and add
    // a database conneciton and JWT key.
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(conn)
        .data(key)
        .data(client)
        // Tracing extension logs query info at the INFO level.
        .extension(Tracing)
        .finish()
}
