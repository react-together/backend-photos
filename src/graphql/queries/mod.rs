use async_graphql::MergedObject;

mod me;
mod user;

use me::BaseQuery;
use user::UserQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(BaseQuery, UserQuery);
