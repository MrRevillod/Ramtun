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

		let token = req
            .cookies()?
            .get("RAMTUN_ACCESS_TOKEN")
            .map(|c| c.value().to_string())
            .ok_or_else(|| {
	            tracing::warn!(method = %method, path = %path, "SessionCheck rejected: missing Access token cookie");
	            JsonResponse::Unauthorized()
            })?;

		let claims: SessionClaims = self
            .jwt_service
            .decode(&token, self.config.jwt_secret.as_ref())
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

	async fn on_connect(&self, socket: SocketContext) -> Result<(), Self::Error> {
		let Some(cookies) = socket.cookies() else {
			tracing::warn!(socket_id = %socket.id(), "SessionCheck rejected: missing cookies");
			return Err("Missing cookies".into());
		};

		let access_cookie = cookies.get("RAMTUN_ACCESS_TOKEN").ok_or_else(|| {
			tracing::warn!(socket_id = %socket.id(), "SessionCheck rejected: missing Access token cookie");
			"Missing Access token cookie".to_string()
		})?;

		let token = access_cookie.value().to_string();

		tracing::debug!(socket_id = %socket.id(), "SessionCheck on_connect: received token");

		let claims: SessionClaims = self
            .jwt_service
            .decode(&token, self.config.jwt_secret.as_ref())
            .inspect_err(|error| {
                tracing::warn!(socket_id = %socket.id(), error = %error, "SessionCheck rejected: token decode failed");
            })
            .map_err(|_| "Token decode failed".to_string())?;

		if claims.typ != "access" {
			tracing::warn!(
				socket_id = %socket.id(),
				session_id = %claims.session_id,
				user_id = %claims.user_id,
				token_type = %claims.typ,
				"SessionCheck rejected: token type is not access"
			);
			return Err("Token type is not access".into());
		}

		let Ok(is_session_active) = self.sessions.is_active(&claims.session_id).await else {
			tracing::warn!(
				socket_id = %socket.id(),
				session_id = %claims.session_id,
				user_id = %claims.user_id,
				"SessionCheck rejected: failed to check session activity"
			);

			return Err("Failed to check session activity".into());
		};

		if !is_session_active {
			tracing::warn!(
				socket_id = %socket.id(),
				session_id = %claims.session_id,
				user_id = %claims.user_id,
				"SessionCheck rejected: session is not active"
			);

			return Err("Session is not active".into());
		}

		tracing::debug!(
			socket_id = %socket.id(),
			session_id = %claims.session_id,
			user_id = %claims.user_id,
			"SessionCheck accepted"
		);

		Ok(())
	}
}
