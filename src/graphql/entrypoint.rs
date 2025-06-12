use crate::{middlewares::user::User, persistances::db};
use async_graphql_rocket::*;
use rocket::http::Status;

pub struct QueryData {
    pub user: User,
}

/// Manually mount an OPTIONS route for your own handling
#[options("/graphql")]
pub fn graphql_options() -> Status {
    Status::Ok
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(request: GraphQLRequest, user: User) -> GraphQLResponse {
    request
        .data(QueryData { user })
        .execute(&super::queries::autoloaded_entities::schema(&**db::get()).unwrap())
        .await
}
