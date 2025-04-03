mod authorization;
mod jwks;
mod jwt;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub iss: String,
    pub sid: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match authorization::from_header(req) {
            Some(token) => token,
            None => {
                return Outcome::Error((Status::Unauthorized, "NoAuthorization".into()));
            }
        };

        let parsed = match jwt::parse::<Token>(&token).await {
            Ok(jwt) => jwt,
            Err(err) => {
                return Outcome::Error((Status::Unauthorized, err.to_string()));
            }
        };

        Outcome::Success(parsed.claims)
    }
}
