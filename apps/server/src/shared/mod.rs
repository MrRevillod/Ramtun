mod cookies;
mod database;
mod errors;
mod extensions;
mod hasher;
mod id;
mod jsonwebtoken;

use database::DatabaseConfig;
use sword::prelude::*;

pub use cookies::*;
pub use database::{Database, TransactionManager, Tx};
pub use errors::*;
pub use extensions::*;
pub use hasher::*;
pub use id::{Entity, Id};
pub use jsonwebtoken::JsonWebTokenService;

pub struct SharedModule;

impl Module for SharedModule {
	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let db_config = config.expect::<DatabaseConfig>();
		let database = Database::new(db_config).await;

		providers.register(database);

		let hasher_config = config.expect::<HasherConfig>();
		let hasher = Hasher::new(&hasher_config);

		providers.register(hasher);
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<JsonWebTokenService>();
		components.register::<TransactionManager>();
	}
}
