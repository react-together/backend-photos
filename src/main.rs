#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::http::Status;
use serde_json::{ json, Value };

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({"hello": "world"}))
}

#[get("/heartbeat")]
fn health_check() -> Status {
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health_check])
}
