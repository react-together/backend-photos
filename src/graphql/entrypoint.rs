use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::State;

use crate::{QueryRoot, middlewares::user::User};

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryData {
    pub user: User,
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<SchemaType>,
    request: GraphQLRequest,
    user: User,
) -> GraphQLResponse {
    request
        .data(QueryData { user })
        .execute(schema.inner())
        .await
}

pub fn build_schema() -> SchemaType {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
