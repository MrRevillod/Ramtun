use crate::auth::AuthError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, de::DeserializeOwned};
use sword::prelude::*;

#[injectable]
pub struct JsonWebTokenService;

impl JsonWebTokenService {
	pub fn encode<C>(&self, claims: &C, key: &[u8]) -> Result<String, AuthError>
	where
		C: Serialize,
	{
		Ok(jsonwebtoken::encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret(key),
		)?)
	}

	pub fn decode<C>(&self, token: &String, key: &[u8]) -> Result<C, AuthError>
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
