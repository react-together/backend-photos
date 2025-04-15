use std::sync::{Arc, OnceLock};

use rocket::Config;
use sea_orm::{ConnectOptions, DatabaseConnection};

use super::config::DatabaseConfig;

static CONNECTION: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();

pub async fn init() -> () {
    let options: ConnectOptions = Config::figment()
        .extract::<DatabaseConfig>()
        .unwrap()
        .url()
        .into();
    let connection = sea_orm::Database::connect(options)
        .await
        .expect("Could not connect to database");

    CONNECTION
        .set(Arc::new(connection))
        .expect("Could not set database connection");
}

pub fn get() -> &'static Arc<DatabaseConnection> {
    CONNECTION
        .get()
        .expect("Database has not been initialized yet")
}
