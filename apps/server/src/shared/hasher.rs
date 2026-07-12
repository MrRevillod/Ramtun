use crate::{auth::AuthError, shared::AppResult};
use serde::Deserialize;
use sword::prelude::*;

#[config(key = "hasher")]
#[derive(Clone, Deserialize)]
pub struct HasherConfig {
	pub cost: u32,
}

#[injectable(provider)]
pub struct Hasher {
	cost: u32,
}

impl Hasher {
	pub fn new(config: &HasherConfig) -> Self {
		Self { cost: config.cost }
	}

	pub async fn hash(&self, password: &str) -> AppResult<String> {
		let cost = self.cost;
		let password = password.to_string();

		let hash = tokio::task::spawn_blocking(move || {
			bcrypt::hash(password, cost).map_err(|e| AuthError::Hashing(e.into()))
		})
		.await
		.map_err(|e| AuthError::Hashing(e.into()))??;

		Ok(hash)
	}

	pub async fn verify(&self, password: &str, hash: &str) -> AppResult<bool> {
		let password = password.to_string();
		let hash = hash.to_string();

		let is_valid = tokio::task::spawn_blocking(move || {
			bcrypt::verify(password, hash.as_str()).map_err(|e| AuthError::Hashing(e.into()))
		})
		.await
		.map_err(|e| AuthError::Hashing(e.into()))??;

		Ok(is_valid)
	}
}

impl Default for HasherConfig {
	fn default() -> Self {
		Self { cost: 12 }
	}
}
