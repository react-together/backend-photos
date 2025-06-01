use async_graphql::{Context, Object};

use crate::entities::*;
use crate::graphql::entrypoint::QueryData;

#[derive(Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<users::Model, async_graphql::Error> {
        Ok(ctx.data::<QueryData>().unwrap().user.from_db.clone())
    }
}
