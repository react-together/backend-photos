use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub sso_audience: Vec<String>,
    pub sso_issuer: Vec<String>,
}
