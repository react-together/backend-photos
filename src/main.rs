#[macro_use]
extern crate rocket;

mod graphql;
mod middlewares;
mod persistances;

use database::migration::{Migrator, MigratorTrait};
use graphql::entrypoint;
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
        .manage(entrypoint::build_schema())
        .mount("/", routes![health_check, entrypoint::graphql_request])
        .attach(AdHoc::config::<persistances::config::AppConfig>());

    #[cfg(debug_assertions)]
    {
        instance.mount("/", routes![graphql::playground::graphql_playground])
    }

    #[cfg(not(debug_assertions))]
    {
        instance
    }
}
