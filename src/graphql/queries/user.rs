use async_graphql::Object;
use sea_orm::{DbErr, EntityTrait};

use crate::entities::{prelude::*, *};
use crate::persistances::db;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> Result<Vec<users::Model>, DbErr> {
        Users::find().all(&**db::get()).await
    }

    async fn user(&self, id: u64) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(&**db::get()).await
    }
}
