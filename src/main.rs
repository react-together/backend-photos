#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde_json::{ json, Value };

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({"hello": "world"}))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
