use crate::auth::*;
use crate::shared::RequestExt;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;
use time::OffsetDateTime as OffsetDT;

#[controller(kind = Controller::Web, path = "/auth")]
pub struct AuthController {
    auth_service: Arc<AuthService>,
}

impl AuthController {
    #[post("/login")]
    pub async fn login(&self, req: Request) -> WebResult {
        let dto = req.body::<LoginDto>()?;

        let LoginResponse {
            user,
            access_token,
            access_token_exp,
            refresh_token,
            refresh_token_exp,
        } = self.auth_service.login(&dto).await?;

        tracing::info!("User login attempt for username: {}", dto.username);

        let Ok(access_exp_dt) = OffsetDT::from_unix_timestamp(access_token_exp.timestamp()) else {
            tracing::error!("Access token expiration Datetime convertion error on login attempt");
            return Err(JsonResponse::InternalServerError());
        };

        let Ok(refresh_exp_dt) = OffsetDT::from_unix_timestamp(refresh_token_exp.timestamp())
        else {
            tracing::error!("Refresh token expiration Datetime convertion error on login attempt");
            return Err(JsonResponse::InternalServerError());
        };

        let access_cookie = CookieBuilder::new("RAMTUN_ACCESS_TOKEN", access_token)
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(CookiesExpiration::DateTime(access_exp_dt))
            .build();

        let refresh_cookie = CookieBuilder::new("RAMTUN_REFRESH_TOKEN", refresh_token)
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(CookiesExpiration::DateTime(refresh_exp_dt))
            .build();

        req.cookies()?.add(access_cookie);
        req.cookies()?.add(refresh_cookie);

        Ok(JsonResponse::Ok().data(user))
    }

    #[post("/refresh")]
    pub async fn refresh(&self, req: Request) -> WebResult {
        let Some(refresh_cookie) = req.cookies()?.get("RAMTUN_REFRESH_TOKEN") else {
            return Err(JsonResponse::Unauthorized());
        };

        let token = refresh_cookie.value().to_string();

        let RefreshResponse {
            access_token,
            access_token_exp,
        } = self.auth_service.refresh(&token).await?;

        let Ok(access_exp_dt) = OffsetDT::from_unix_timestamp(access_token_exp.timestamp()) else {
            tracing::error!("Access token expiration Datetime conversion error on refresh");
            return Err(JsonResponse::InternalServerError());
        };

        let access_cookie = CookieBuilder::new("RAMTUN_ACCESS_TOKEN", access_token)
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(CookiesExpiration::DateTime(access_exp_dt))
            .build();

        req.cookies()?.add(access_cookie);

        Ok(JsonResponse::Ok())
    }

    #[post("/logout")]
    #[interceptor(SessionCheck)]
    pub async fn logout(&self, req: Request) -> WebResult {
        let session_claims = req.claims().ok_or_else(JsonResponse::Unauthorized)?;

        self.auth_service.logout(&session_claims.session_id).await?;

        req.cookies()?
            .remove(Cookie::new("RAMTUN_ACCESS_TOKEN", ""));

        req.cookies()?
            .remove(Cookie::new("RAMTUN_REFRESH_TOKEN", ""));

        Ok(JsonResponse::Ok().message("Sesión cerrada correctamente"))
    }
}
