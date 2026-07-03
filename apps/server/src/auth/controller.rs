use crate::auth::*;
use crate::shared::{CookieManager, RequestExt};

use chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = Controller::Web, path = "/auth")]
pub struct AuthController {
    auth_service: Arc<AuthService>,
    cookie_manager: Arc<CookieManager>,
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

        let access_cookie = self
            .cookie_manager
            .build_access_cookie(access_token, access_token_exp)?;

        let refresh_cookie = self
            .cookie_manager
            .build_refresh_cookie(refresh_token, refresh_token_exp)?;

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

        let access_cookie = self
            .cookie_manager
            .build_access_cookie(access_token, access_token_exp)?;

        req.cookies()?.add(access_cookie);

        Ok(JsonResponse::Ok())
    }

    #[post("/logout")]
    #[interceptor(SessionCheck)]
    pub async fn logout(&self, req: Request) -> WebResult {
        let session_claims = req.claims().ok_or_else(JsonResponse::Unauthorized)?;

        self.auth_service.logout(&session_claims.session_id).await?;

        let access_cookie = self
            .cookie_manager
            .build_access_cookie("".into(), Utc::now())?;

        let refresh_cookie = self
            .cookie_manager
            .build_refresh_cookie("".into(), Utc::now())?;

        req.cookies()?.remove(access_cookie);
        req.cookies()?.remove(refresh_cookie);

        Ok(JsonResponse::Ok().message("Sesión cerrada correctamente"))
    }
}
