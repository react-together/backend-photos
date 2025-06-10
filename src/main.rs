#[macro_use]
extern crate rocket;

mod entities;
mod graphql;
mod middlewares;
mod persistances;

use graphql::entrypoint;
use migration::{Migrator, MigratorTrait};
use persistances::db;
use rocket::{Build, Rocket, fairing::AdHoc, http::Status};

#[get("/heartbeat")]
fn health_check() -> Status {
    Status::NoContent
}

#[launch]
async fn rocket() -> Rocket<Build> {
    db::init().await;

    let _ = Migrator::up(&**db::get(), None).await;

    let instance = rocket::build()
        .mount("/", routes![health_check, entrypoint::graphql_request])
        .attach(AdHoc::config::<persistances::config::AppConfig>())
        .attach(middlewares::cors::CORS);

    #[cfg(debug_assertions)]
    {
        instance.mount("/", routes![graphql::playground::graphql_playground])
    }

    #[cfg(not(debug_assertions))]
    {
        instance
    }
}
