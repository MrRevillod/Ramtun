use crate::shared::{AppError, AppResult};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use sword::prelude::*;
use sword::web::{Cookie, CookieBuilder, CookiesExpiration, SameSite};
use time::OffsetDateTime;

pub const ACTIVE_ATTEMPT_COOKIE: &str = "RAMTUN_ACTIVE_ATTEMPT";

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

    pub fn build_active_attempt_cookie(
        &self,
        attempt_id: &str,
        exp: DateTime<Utc>,
    ) -> AppResult<Cookie<'static>> {
        let expiration = self.format_expiration(exp)?;

        let cookie = CookieBuilder::new(ACTIVE_ATTEMPT_COOKIE.to_string(), attempt_id.to_string())
            .http_only(self.config.http_only)
            .secure(self.config.secure)
            .same_site(SameSite::Strict)
            .path(self.config.path.clone())
            .expires(expiration)
            .build();

        Ok(cookie)
    }

    pub fn clear_active_attempt_cookie(&self) -> AppResult<Cookie<'static>> {
        let expiration = CookiesExpiration::DateTime(
            OffsetDateTime::from_unix_timestamp(0).unwrap_or_else(|_| OffsetDateTime::now_utc()),
        );

        let cookie = CookieBuilder::new(ACTIVE_ATTEMPT_COOKIE.to_string(), String::new())
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
