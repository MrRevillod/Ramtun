use crate::{
    shared::{AppResult, Database},
    users::{User, UserRole},
};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub async fn find_by_id(&self, user_id: &uuid::Uuid) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
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
            "INSERT INTO users (id, username, name, email, role)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (username) DO UPDATE
             SET name = EXCLUDED.name,
                 email = EXCLUDED.email,
                 role = EXCLUDED.role
             RETURNING *",
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.role)
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(result)
    }

    pub async fn list_users(
        &self,
        query: Option<&str>,
        roles: Option<&str>,
    ) -> AppResult<Vec<User>> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM users WHERE 1=1");

        if let Some(q) = query {
            let pattern = format!("%{}%", q.trim());

            qb.push(" AND (username ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR name ILIKE ")
                .push_bind(pattern)
                .push(")");
        }

        if let Some(roles_raw) = roles {
            let roles: Vec<UserRole> = roles_raw
                .split(',')
                .filter_map(|role| match role.trim().to_lowercase().as_str() {
                    "student" => Some(UserRole::Student),
                    "assistant" => Some(UserRole::Assistant),
                    "func" => Some(UserRole::Func),
                    _ => None,
                })
                .collect();

            if !roles.is_empty() {
                qb.push(" AND role IN (");

                let mut separated = qb.separated(", ");
                for role in roles {
                    separated.push_bind(role);
                }

                separated.push_unseparated(")");
            }
        }

        qb.push(" ORDER BY username ASC LIMIT 200");

        let users = qb
            .build_query_as::<User>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(users)
    }
}
