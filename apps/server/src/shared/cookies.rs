use crate::shared::{AppError, AppResult};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use sword::prelude::*;
use sword::web::{Cookie, CookieBuilder, CookiesExpiration, SameSite};
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "cookies")]
pub struct CookieConfig {
    http_only: bool,
    secure: bool,
    path: String,
    access_cookie_name: String,
    refresh_cookie_name: String,
}

#[injectable]
pub struct CookieManager {
    config: CookieConfig,
}

impl CookieManager {
    pub fn build_access_cookie(
        &self,
        value: String,
        exp: DateTime<Utc>,
    ) -> AppResult<Cookie<'static>> {
        let expiration = self.format_expiration(exp)?;

        let cookie = CookieBuilder::new(self.config.access_cookie_name.clone(), value)
            .http_only(self.config.http_only)
            .secure(self.config.secure)
            .same_site(SameSite::Strict)
            .path(self.config.path.clone())
            .expires(expiration)
            .build();

        Ok(cookie)
    }

    pub fn build_refresh_cookie(
        &self,
        value: String,
        exp: DateTime<Utc>,
    ) -> AppResult<Cookie<'static>> {
        let expiration = self.format_expiration(exp)?;

        let cookie = CookieBuilder::new(self.config.refresh_cookie_name.clone(), value)
            .http_only(self.config.http_only)
            .secure(self.config.secure)
            .same_site(SameSite::Strict)
            .path(self.config.path.clone())
            .expires(expiration)
            .build();

        Ok(cookie)
    }

    pub fn format_expiration(&self, expires: DateTime<Utc>) -> AppResult<CookiesExpiration> {
        let Ok(exp_dt) = OffsetDateTime::from_unix_timestamp(expires.timestamp()) else {
            tracing::error!("Cookie expiration Datetime convertion error on CookieBuilding");
            return Err(AppError::InternalError);
        };

        Ok(CookiesExpiration::DateTime(exp_dt))
    }
}
