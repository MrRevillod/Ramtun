use crate::{
    auth::{AuthConfig, SessionClaims, SessionRepository},
    shared::JsonWebTokenService,
};

use std::sync::Arc;
use sword::socketio::*;
use sword::web::*;
use sword::{prelude::*, socketio::OnConnect};

#[derive(Interceptor)]
pub struct SessionCheck {
    config: AuthConfig,
    jwt_service: Arc<JsonWebTokenService>,
    sessions: Arc<SessionRepository>,
}

impl OnRequest for SessionCheck {
    async fn on_request(&self, mut req: Request) -> WebInterceptorResult {
        let method = req.method().to_string();
        let path = req.uri();

        let Some(auth_header) = req.authorization() else {
            tracing::warn!(method = %method, path = %path, "SessionCheck rejected: missing Authorization header");
            return Err(JsonResponse::Unauthorized());
        };

        let Some(token) = auth_header.strip_prefix("Bearer ") else {
            tracing::warn!(method = %method, path = %path, "SessionCheck rejected: invalid Authorization scheme");
            return Err(JsonResponse::Unauthorized());
        };

        let claims: SessionClaims = self
            .jwt_service
            .decode(&token.to_string(), self.config.jwt_secret.as_ref())
            .inspect_err(|error| {
                tracing::warn!(method = %method, path = %path, error = %error, "SessionCheck rejected: token decode failed");
            })?;

        if claims.typ != "access" {
            tracing::warn!(
                method = %method,
                path = %path,
                session_id = %claims.session_id,
                user_id = %claims.user_id,
                token_type = %claims.typ,
                "SessionCheck rejected: token type is not access"
            );
            return Err(JsonResponse::Unauthorized());
        }

        if !self.sessions.is_active(&claims.session_id).await? {
            tracing::warn!(
                method = %method,
                path = %path,
                session_id = %claims.session_id,
                user_id = %claims.user_id,
                "SessionCheck rejected: session is not active"
            );
            return Err(JsonResponse::Unauthorized());
        }

        tracing::debug!(
            method = %method,
            path = %path,
            session_id = %claims.session_id,
            user_id = %claims.user_id,
            "SessionCheck accepted"
        );

        req.extensions.insert(claims);

        req.next().await
    }
}

impl OnConnect for SessionCheck {
    type Error = String;

    async fn on_connect(&self, ctx: SocketContext) -> Result<(), Self::Error> {
        let token = ctx
            .authorization()
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_string())
            .or_else(|| {
                ctx.socket.req_parts().uri.query().and_then(|q| {
                    q.split('&')
                        .find_map(|pair| pair.split_once('='))
                        .filter(|(key, _)| *key == "token")
                        .map(|(_, value)| value.to_string())
                })
            });

        let Some(ref token) = token else {
            tracing::warn!(socket_id = %ctx.id(), "SessionCheck rejected: missing token");
            return Err("Missing token".into());
        };

        tracing::debug!(socket_id = %ctx.id(), "SessionCheck on_connect: received token");

        let claims: SessionClaims = self
            .jwt_service
            .decode(token, self.config.jwt_secret.as_ref())
            .inspect_err(|error| {
                tracing::warn!(socket_id = %ctx.id(), error = %error, "SessionCheck rejected: token decode failed");
            })
            .map_err(|_| "Token decode failed".to_string())?;

        if claims.typ != "access" {
            tracing::warn!(
                socket_id = %ctx.id(),
                session_id = %claims.session_id,
                user_id = %claims.user_id,
                token_type = %claims.typ,
                "SessionCheck rejected: token type is not access"
            );
            return Err("Token type is not access".into());
        }

        let Ok(is_session_active) = self.sessions.is_active(&claims.session_id).await else {
            tracing::warn!(
                socket_id = %ctx.id(),
                session_id = %claims.session_id,
                user_id = %claims.user_id,
                "SessionCheck rejected: failed to check session activity"
            );

            return Err("Failed to check session activity".into());
        };

        if !is_session_active {
            tracing::warn!(
                socket_id = %ctx.id(),
                session_id = %claims.session_id,
                user_id = %claims.user_id,
                "SessionCheck rejected: session is not active"
            );

            return Err("Session is not active".into());
        }

        tracing::debug!(
            socket_id = %ctx.id(),
            session_id = %claims.session_id,
            user_id = %claims.user_id,
            "SessionCheck accepted"
        );

        Ok(())
    }
}
