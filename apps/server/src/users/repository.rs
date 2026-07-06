use crate::{
    shared::{AppResult, Database},
    users::{User, UserFilter, UserId, UserView},
};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub async fn find_by_id(&self, user_id: &UserId) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(self.db.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(self.db.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn save(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, name, email, password_hash, role, last_login_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (username) DO UPDATE
             SET name = EXCLUDED.name,
                 email = EXCLUDED.email,
                 role = EXCLUDED.role,
                 password_hash = EXCLUDED.password_hash,
                 last_login_at = EXCLUDED.last_login_at
             RETURNING *",
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(user.last_login_at)
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(result)
    }

    pub async fn list_users(&self, filter: UserFilter) -> AppResult<Vec<UserView>> {
        let mut qb: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT id, name, username, email, role FROM users WHERE 1=1");

        if let Some(q) = filter.search {
            let pattern = format!("%{}%", q.trim());

            qb.push(" AND (username ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR name ILIKE ")
                .push_bind(pattern)
                .push(")");
        }

        if let Some(roles) = filter.roles
            && !roles.is_empty()
        {
            qb.push(" AND role IN (");

            let mut separated = qb.separated(", ");
            for role in roles {
                separated.push_bind(role);
            }

            separated.push_unseparated(")");
        }

        qb.push(" ORDER BY username ASC LIMIT 200");

        let users = qb
            .build_query_as::<UserView>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(users)
    }
}
