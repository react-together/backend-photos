#[macro_use]
extern crate rocket;

mod middlewares;
mod persistances;

use middlewares::keycloak::Token;
use rocket::{ fairing::AdHoc, http::Status, serde::json::Json };
use serde_json::{ Value, json };

#[get("/")]
fn index(token: Token) -> Json<Value> {
    Json(json!({"hello": "world", "token": token}))
}

#[get("/heartbeat")]
fn health_check() -> Status {
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket
        ::build()
        .mount("/", routes![index, health_check])
        .attach(AdHoc::config::<persistances::config::AppConfig>())
}
