mod cookies;
mod database;
mod errors;
mod event_queue;
mod extensions;
mod id;
mod jsonwebtoken;
mod mailer;

use database::DatabaseConfig;
use sword::prelude::*;

pub use cookies::*;
pub use database::{Database, TransactionManager, Tx};
pub use errors::*;
pub use event_queue::*;
pub use extensions::*;
pub use id::{Entity, Id};
pub use jsonwebtoken::JsonWebTokenService;
pub use mailer::*;

pub struct SharedModule;

impl Module for SharedModule {
    async fn register_providers(config: &Config, providers: &ProviderRegistry) {
        let db_config = config.expect::<DatabaseConfig>();
        let database = Database::new(db_config).await;

        providers.register(database);
    }

    fn register_components(components: &ComponentRegistry) {
        components.register::<JsonWebTokenService>();
        components.register::<TransactionManager>();
    }
}
