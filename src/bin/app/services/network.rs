use std::sync::Arc;
use graphql_client::{GraphQLQuery};
use anyhow::Result;

pub async fn post_graphql<Q:GraphQLQuery>(vars:<Q as GraphQLQuery>::Variables)
                                          -> Result<Arc<graphql_client::Response<Q::ResponseData>>>{
    // We need an Arc here because we want to call it from use_async,
    // response is not clone and use_async's future state require clones? (I think, not sure)
    Ok(Arc::new(
        gloo::net::http::Request::post("api/graphql")
            //Facebook uses JSON for network requests in part because it's amiable to gzip
            .header("accept-encoding","gzip")
            //Turns our variables into a GraphQL query JSON formatted string.
            .json(&Q::build_query(vars))?
            .send()
            .await?
            .json()
            .await?
    ))
}