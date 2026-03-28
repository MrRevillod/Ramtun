use crate::{
    auth::{AuthConfig, SessionClaims, SessionRepository},
    shared::JsonWebTokenService,
};
use std::sync::Arc;
use sword::prelude::*;

#[derive(Interceptor)]
pub struct SessionCheck {
    config: AuthConfig,
    jwt_service: Arc<JsonWebTokenService>,
    sessions: Arc<SessionRepository>,
}

impl OnRequest for SessionCheck {
    async fn on_request(&self, mut req: Request) -> HttpInterceptorResult {
        let token = Self::extract_bearer_token(req.authorization())?;

        let claims: SessionClaims = self
            .jwt_service
            .decode(&token, self.config.jwt_secret.as_ref())?;

        if claims.typ != "access" {
            return Err(JsonResponse::Unauthorized());
        }

        if !self.sessions.is_active(&claims.session_id).await? {
            return Err(JsonResponse::Unauthorized());
        }

        req.extensions.insert(claims);

        req.next().await
    }
}

impl SessionCheck {
    pub fn extract_bearer_token(auth_header: Option<&str>) -> HttpResult<String> {
        let auth_header = auth_header.ok_or_else(JsonResponse::Unauthorized)?;

        let Some(token) = auth_header.strip_prefix("Bearer ") else {
            return Err(JsonResponse::Unauthorized());
        };

        Ok(token.to_string())
    }
}
