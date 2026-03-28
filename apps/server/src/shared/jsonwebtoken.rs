use crate::shared::AppResult;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, de::DeserializeOwned};
use sword::prelude::*;

#[injectable]
pub struct JsonWebTokenService;

impl JsonWebTokenService {
    pub fn encode<C>(&self, claims: &C, key: &[u8]) -> AppResult<String>
    where
        C: Serialize,
    {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(key),
        )?)
    }

    pub fn decode<C>(&self, token: &String, key: &[u8]) -> AppResult<C>
    where
        C: DeserializeOwned,
    {
        let decoded = jsonwebtoken::decode::<C>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::default(),
        )?;

        Ok(decoded.claims)
    }
}
