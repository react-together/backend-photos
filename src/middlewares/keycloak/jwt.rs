use core::fmt;

use crate::{middlewares::keycloak::jwks, persistances::config::AppConfig};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use rocket::Config;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum ParseError {
    JwtParseError(jsonwebtoken::errors::Error),
    MissingIssuer,
    JwkFetchError(reqwest::Error),
    JwkParseError(jsonwebtoken::errors::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::JwtParseError(err) => write!(f, "JwtParseError {err}"),
            ParseError::MissingIssuer => write!(f, "MissingIssuer"),
            ParseError::JwkFetchError(err) => write!(f, "JwkFetchError {err}"),
            ParseError::JwkParseError(err) => write!(f, "JwkParseError {err}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    // pub exp: usize,
    // pub iat: usize,
    // pub jti: String,
    pub iss: String,
    // pub typ: String,
    // pub sid: String,
}

fn get_validator(algorithm: &Algorithm) -> Validation {
    let mut validator = Validation::new(*algorithm);
    let app_config = Config::figment().extract::<AppConfig>().unwrap();
    validator.set_audience(&app_config.sso_audience);
    validator.set_issuer(&app_config.sso_issuer);

    validator
}

fn get_issuer(
    token: &String,
    algorithm: &Algorithm,
) -> Result<String, jsonwebtoken::errors::Error> {
    let empty_key = DecodingKey::from_secret(&[]);
    let mut validation = get_validator(algorithm);
    validation.insecure_disable_signature_validation();

    match jsonwebtoken::decode::<JwtClaims>(token, &empty_key, &validation) {
        Ok(data) => Ok(data.claims.iss),
        Err(err) => Err(err),
    }
}

pub async fn parse<T: DeserializeOwned>(token: &String) -> Result<TokenData<T>, ParseError> {
    let header =
        jsonwebtoken::decode_header(&token).or_else(|err| Err(ParseError::JwtParseError(err)))?;

    let issuer =
        get_issuer(&token, &header.alg).or_else(|err| Err(ParseError::JwtParseError(err)))?;

    let kid = header.kid.ok_or(ParseError::MissingIssuer)?;

    let jwk = jwks::get_jwk(&kid, &issuer)
        .await
        .or_else(|err| Err(ParseError::JwkFetchError(err)))?;

    let decoding_key =
        DecodingKey::from_jwk(&jwk).or_else(|err| Err(ParseError::JwkParseError(err)))?;

    jsonwebtoken::decode::<T>(&token, &decoding_key, &get_validator(&header.alg))
        .or_else(|err| Err(ParseError::JwtParseError(err)))
}
