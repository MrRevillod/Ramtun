use crate::{
    shared::{Entity, Id},
    users::{User, UserId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub type SessionId = Id<Session>;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub refresh_token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl Entity for Session {
    fn key_name() -> &'static str {
        "session"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub session_id: SessionId,
    pub user_id: UserId,
    pub exp: i64,
    pub typ: String,
}
