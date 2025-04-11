#[macro_use]
extern crate rocket;

mod middlewares;
mod persistances;

use database::{
    entity::*,
    migration::{self, MigratorTrait},
};
use middlewares::{database::DatabaseClient, keycloak::Token};
use rocket::{
    Build, Rocket,
    fairing::{self, AdHoc},
    http::Status,
    serde::json::Json,
};
use sea_orm::EntityTrait;
use sea_orm_rocket::{Connection, Database};
use serde_json::{Value, json};

#[get("/")]
async fn index(token: Token, conn: Connection<'_, DatabaseClient>) -> Json<Value> {
    let users = users::Entity::find().all(conn.into_inner()).await.unwrap();

    Json(json!({"hello": "world", "users": users, "token": token}))
}

#[get("/heartbeat")]
fn health_check() -> Status {
    Status::NoContent
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &DatabaseClient::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DatabaseClient::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![index, health_check])
        .attach(AdHoc::config::<persistances::config::AppConfig>())
}
