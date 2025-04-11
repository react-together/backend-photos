use async_trait::async_trait;
use rocket::Config;
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use sea_orm_rocket::{Pool, rocket::figment::Figment};

use crate::persistances::config::DatabaseConfig;

#[derive(sea_orm_rocket::Database, Debug)]
#[database("sea_orm")]
pub struct DatabaseClient(DatabasePool);

#[derive(Debug, Clone)]
pub struct DatabasePool {
    pub conn: DatabaseConnection,
}

#[async_trait]
impl Pool for DatabasePool {
    type Error = DbErr;

    type Connection = DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let config = Config::figment().extract::<DatabaseConfig>().unwrap();
        let options: ConnectOptions = config.url().into();
        let conn = sea_orm::Database::connect(options).await?;

        Ok(DatabasePool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}
