use std::sync::LazyLock;

use jsonwebtoken::jwk::{ Jwk, JwkSet };
use moka::future::Cache;
use reqwest::Client;

static JWKS_CACHE: LazyLock<Cache<String, JwkSet>> = LazyLock::new(|| Cache::new(100));

async fn fetch_jwks(issuer_base_url: &str) -> Result<JwkSet, reqwest::Error> {
    let client = Client::new();
    let jwks_url = format!(
        "{}/protocol/openid-connect/certs",
        issuer_base_url.trim_end_matches('/')
    );

    let jwk_set = client.get(&jwks_url).send().await?.json::<JwkSet>().await?;
    JWKS_CACHE.insert(issuer_base_url.to_string(), jwk_set.clone()).await;

    Ok(jwk_set)
}

pub async fn get_jwks(issuer_base_url: &str) -> Result<JwkSet, reqwest::Error> {
    if let Some(jwk_set) = JWKS_CACHE.get(issuer_base_url).await {
        return Ok(jwk_set);
    }

    fetch_jwks(issuer_base_url).await
}

pub async fn get_jwk(issuer_base_url: &str, kid: &str) -> Result<Jwk, reqwest::Error> {
    let jwks = get_jwks(issuer_base_url).await?;

    Ok(jwks.find(kid).unwrap().clone())
}
