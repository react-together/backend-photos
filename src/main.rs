#[macro_use]
extern crate rocket;

mod middlewares;
mod persistances;

use database::{
    entity::*,
    migration::{Migrator, MigratorTrait},
};
use middlewares::keycloak::Token;
use persistances::db;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use sea_orm::EntityTrait;
use serde_json::{Value, json};

#[get("/")]
async fn index(token: Token) -> Json<Value> {
    let users = users::Entity::find().all(&**db::get()).await.unwrap();

    Json(json!({"hello": "world", "users": users, "token": token}))
}

#[get("/heartbeat")]
fn health_check() -> Status {
    Status::NoContent
}

#[launch]
async fn rocket() -> _ {
    db::init().await;

    let _ = Migrator::up(&**db::get(), None).await;

    rocket::build()
        .mount("/", routes![index, health_check])
        .attach(AdHoc::config::<persistances::config::AppConfig>())
}
