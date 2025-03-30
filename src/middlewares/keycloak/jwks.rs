use std::sync::LazyLock;

use jsonwebtoken::jwk::{ Jwk, JwkSet };
use moka::future::Cache;
use reqwest::Client;

static JWK_CACHE: LazyLock<Cache<String, Jwk>> = LazyLock::new(|| Cache::new(100));

async fn fetch_jwks(issuer_base_url: &str) -> Result<JwkSet, reqwest::Error> {
    let client = Client::new();
    let jwks_url = format!(
        "{}/protocol/openid-connect/certs",
        issuer_base_url.trim_end_matches('/')
    );

    let jwk_set = client.get(&jwks_url).send().await?.json::<JwkSet>().await?;

    // Cache each individual JWK by its kid
    for jwk in &jwk_set.keys {
        if let Some(kid) = &jwk.common.key_id {
            JWK_CACHE.insert(kid.clone(), jwk.clone()).await;
        }
    }

    Ok(jwk_set)
}

pub async fn get_jwk(kid: &str, issuer_base_url: &str) -> Result<Jwk, reqwest::Error> {
    // First try to get from cache
    if let Some(jwk) = JWK_CACHE.get(kid).await {
        return Ok(jwk);
    }

    // If not in cache, fetch all JWKs and try again
    fetch_jwks(issuer_base_url).await?;

    // Now it should be in cache
    Ok(JWK_CACHE.get(kid).await.unwrap().clone())
}
