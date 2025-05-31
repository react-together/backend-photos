use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use rocket::response::content::RawHtml;

#[rocket::get("/graphql")]
pub fn graphql_playground() -> RawHtml<String> {
    RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
