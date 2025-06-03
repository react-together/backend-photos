use async_graphql::Object;
use sea_orm::{DbErr, EntityTrait};

use crate::entities::{prelude::*, *};
use crate::persistances::db;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(&**db::get()).await
    }

    async fn user(&self, id: u64) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(&**db::get()).await
    }
}
