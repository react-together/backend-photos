use jsonwebtoken::jwk::{ Jwk, JwkSet };
use reqwest::Client;

pub async fn fetch_jwks(issuer_base_url: &String) -> Result<JwkSet, reqwest::Error> {
    let client = Client::new();
    let jwks_url = format!(
        "{}/protocol/openid-connect/certs",
        issuer_base_url.trim_end_matches('/')
    );

    client.get(jwks_url).send().await?.json::<JwkSet>().await
}

pub async fn fetch_jwk(issuer_base_url: &String, kid: &String) -> Result<Jwk, reqwest::Error> {
    let jwks = fetch_jwks(issuer_base_url).await?;

    Ok(jwks.find(kid).unwrap().clone())
}
