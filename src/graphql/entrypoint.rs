use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::State;

use crate::QueryRoot;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<SchemaType>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

pub fn build_schema() -> SchemaType {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
