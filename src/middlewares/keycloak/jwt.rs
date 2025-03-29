use core::fmt;

use jsonwebtoken::{ Algorithm, DecodingKey, TokenData, Validation, decode };
use serde::{ Deserialize, Serialize, de::DeserializeOwned };

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

fn get_issuer(
    token: &String,
    algorithm: &Algorithm
) -> Result<String, jsonwebtoken::errors::Error> {
    let empty_key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(*algorithm);
    validation.insecure_disable_signature_validation();

    match decode::<JwtClaims>(token, &empty_key, &validation) {
        Ok(data) => Ok(data.claims.iss),
        Err(err) => Err(err),
    }
}

pub async fn parse<T: DeserializeOwned>(token: &String) -> Result<TokenData<T>, ParseError> {
    let header = match jsonwebtoken::decode_header(&token) {
        Ok(header) => header,
        Err(err) => {
            return Err(ParseError::JwtParseError(err));
        }
    };

    let issuer = match get_issuer(&token, &header.alg) {
        Ok(issuer) => issuer,
        Err(err) => {
            return Err(ParseError::JwtParseError(err));
        }
    };

    let kid = match header.kid {
        Some(kid) => kid,
        None => {
            return Err(ParseError::MissingIssuer);
        }
    };

    let jwk = match crate::middlewares::keycloak::jwks::fetch_jwk(&issuer, &kid).await {
        Ok(jwk) => jwk,
        Err(err) => {
            return Err(ParseError::JwkFetchError(err));
        }
    };

    let decoding_key = match DecodingKey::from_jwk(&jwk) {
        Ok(key) => key,
        Err(err) => {
            return Err(ParseError::JwkParseError(err));
        }
    };

    match jsonwebtoken::decode::<T>(&token, &decoding_key, &Validation::new(Algorithm::RS256)) {
        Ok(token_info) => Ok(token_info),
        Err(err) => Err(ParseError::JwtParseError(err)),
    }
}
