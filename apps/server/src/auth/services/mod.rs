mod ldap;

use crate::{
    auth::*,
    shared::*,
    users::{User, UserId, UserRepository},
};

use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use sword::prelude::*;

pub use ldap::LdapClient;

#[injectable]
pub struct AuthService {
    config: AuthConfig,
    ldap: Arc<LdapClient>,
    users: Arc<UserRepository>,
    jwt_service: Arc<JsonWebTokenService>,
    sessions: Arc<SessionRepository>,
    hasher: Arc<Hasher>,
}

impl AuthService {
    pub async fn login(&self, input: &LoginDto) -> AppResult<LoginResponse> {
        let LoginDto { username, password } = input;

        let mut user = match self.users.find_by_username(username).await? {
            Some(mut user) => {
                if !self.hasher.verify(password, &user.password_hash).await? {
                    self.ldap.authenticate(username, password).await?;
                    user.password_hash = self.hasher.hash(password).await?;
                }

                user
            }
            None => {
                let ldap_user = self.ldap.authenticate(username, password).await?;

                let incoming_user = User::builder()
                    .username(username.clone())
                    .email(ldap_user.email)
                    .name(ldap_user.name)
                    .role(ldap_user.role)
                    .password_hash(self.hasher.hash(password).await?)
                    .build();

                self.users.save(&incoming_user).await?
            }
        };

        let session_id = SessionId::new();

        let (access_token, access_token_exp) = self.generate_access_token(&session_id, &user.id)?;
        let (refresh_token, refresh_token_exp) =
            self.generate_refresh_token(&session_id, &user.id)?;

        let now = Utc::now();

        let session = Session {
            id: session_id,
            user_id: user.id,
            refresh_token_hash: Self::hash_token(&refresh_token),
            created_at: now,
            expires_at: access_token_exp,
            refresh_expires_at: refresh_token_exp,
            revoked_at: None,
        };

        user.last_login_at = Some(now);

        self.sessions.save(&session).await?;
        self.users.save(&user).await?;

        Ok(LoginResponse {
            user: user.into(),
            access_token,
            access_token_exp,
            refresh_token,
            refresh_token_exp,
        })
    }

    pub async fn refresh(&self, token: &String) -> AppResult<RefreshResponse> {
        let claims = self
            .jwt_service
            .decode::<SessionClaims>(token, self.config.jwt_secret.as_ref())?;

        if claims.typ != "refresh" {
            Err(AuthError::InvalidToken)?;
        }

        let session = self
            .sessions
            .find_active_by_refresh_id(&claims.session_id)
            .await?
            .ok_or(AuthError::TokenNotFound)?;

        let (access_token, access_token_exp) =
            self.generate_access_token(&session.id, &session.user_id)?;

        self.sessions
            .update_expires_at(&session.id, access_token_exp)
            .await?;

        Ok(RefreshResponse {
            access_token,
            access_token_exp,
        })
    }

    pub async fn logout(&self, session_id: &SessionId) -> AppResult<()> {
        if let Some(mut session) = self.sessions.find_active_by_id(session_id).await? {
            session.revoked_at = Some(Utc::now());
            self.sessions.save(&session).await?;
        }

        Ok(())
    }

    fn generate_access_token(
        &self,
        session_id: &SessionId,
        user_id: &UserId,
    ) -> AppResult<(String, DateTime<Utc>)> {
        let expiration = Utc::now() + Duration::minutes(self.config.access_exp_minutes);

        let claims = SessionClaims {
            session_id: *session_id,
            user_id: *user_id,
            exp: expiration.timestamp(),
            typ: "access".to_string(),
        };

        let token = self
            .jwt_service
            .encode(&claims, self.config.jwt_secret.as_ref())?;

        Ok((token, expiration))
    }

    fn generate_refresh_token(
        &self,
        session_id: &SessionId,
        user_id: &UserId,
    ) -> AppResult<(String, DateTime<Utc>)> {
        let expiration = Utc::now() + Duration::days(self.config.refresh_exp_days);

        let claims = SessionClaims {
            session_id: *session_id,
            user_id: *user_id,
            exp: expiration.timestamp(),
            typ: "refresh".to_string(),
        };

        let token = self
            .jwt_service
            .encode(&claims, self.config.jwt_secret.as_ref())?;

        Ok((token, expiration))
    }

    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
