use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub sso_audience: Vec<String>,
    pub sso_issuer: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_database: String,
}

impl DatabaseConfig {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_database
        )
    }
}
