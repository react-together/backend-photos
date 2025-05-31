use async_graphql::{Context, Object};
use database::entity::{prelude::*, *};
use sea_orm::{DbErr, EntityTrait};

use crate::persistances::db;

use super::entrypoint::QueryData;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> Result<users::Model, async_graphql::Error> {
        Ok(ctx.data::<QueryData>().unwrap().user.from_db.clone())
    }

    async fn users(&self) -> Result<Vec<users::Model>, DbErr> {
        Users::find().all(&**db::get()).await
    }

    async fn user(&self, id: u64) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(&**db::get()).await
    }
}
