use async_graphql::{Context, Object};

use crate::graphql::entrypoint::QueryData;
use database::entity::*;

#[derive(Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<users::Model, async_graphql::Error> {
        Ok(ctx.data::<QueryData>().unwrap().user.from_db.clone())
    }
}
