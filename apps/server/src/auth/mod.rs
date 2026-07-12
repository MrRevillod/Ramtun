mod controller;
mod dtos;
mod errors;
mod interceptor;
mod repository;
mod services;

use crate::shared::CookieManager;
use controller::AuthController;
use serde::Deserialize;
use sword::prelude::*;

pub use dtos::*;
pub use errors::AuthError;
pub use interceptor::SessionCheck;
pub use repository::SessionRepository;
pub use services::{AuthService, LdapClient};

#[config(key = "auth")]
#[derive(Clone, Deserialize)]
pub struct AuthConfig {
	pub ldap_url: String,
	pub ldap_admin_user: String,
	pub ldap_admin_password: String,
	pub ldap_base_dn: String,
	pub access_exp_minutes: i64,
	pub refresh_exp_days: i64,
	pub jwt_secret: String,
	pub password_sync_api_key: String,
}

pub struct AuthModule;

impl Module for AuthModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<AuthController>();
	}

	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let auth_config = config.expect::<AuthConfig>();

		let ldap_client = LdapClient::new(auth_config.clone())
			.await
			.expect("Failed to create LDAP client");

		providers.register(ldap_client);
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<AuthService>();
		components.register::<SessionRepository>();
		components.register::<CookieManager>();
	}
}
