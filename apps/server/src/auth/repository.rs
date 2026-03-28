use crate::{
    auth::Session,
    shared::{AppResult, Database},
};
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct SessionRepository {
    database: Arc<Database>,
}

impl SessionRepository {
    pub async fn save(&self, session: &Session) -> AppResult<Session> {
        let session = sqlx::query_as::<_, Session>(
            "INSERT INTO sessions (
                id,
                user_id,
                refresh_token_hash,
                created_at,
                expires_at,
                refresh_expires_at,
                revoked_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                refresh_token_hash = EXCLUDED.refresh_token_hash,
                created_at = EXCLUDED.created_at,
                expires_at = EXCLUDED.expires_at,
                refresh_expires_at = EXCLUDED.refresh_expires_at,
                revoked_at = EXCLUDED.revoked_at
            RETURNING *",
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.refresh_token_hash.clone())
        .bind(session.created_at)
        .bind(session.expires_at)
        .bind(session.refresh_expires_at)
        .bind(session.revoked_at)
        .fetch_one(self.database.get_pool())
        .await?;

        Ok(session)
    }

    pub async fn is_active(&self, id: &Uuid) -> AppResult<bool> {
        let res = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions
             WHERE id = $1
             AND revoked_at IS NULL
             AND expires_at > NOW()",
        )
        .bind(id)
        .fetch_optional(self.database.get_pool())
        .await?;

        Ok(res.is_some())
    }

    pub async fn find_active_by_id(&self, id: &Uuid) -> AppResult<Option<Session>> {
        let res = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions
             WHERE id = $1
             AND revoked_at IS NULL
             AND expires_at > NOW()",
        )
        .bind(id)
        .fetch_optional(self.database.get_pool())
        .await?;

        Ok(res)
    }
}
