use crate::{shared::AppResult, users::*};
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct UsersService {
    policy: UserPolicy,
    users: UserRepository,
}

impl UsersService {
    pub async fn list_users_admin(&self, query: SearchUsersQuery) -> AppResult<Vec<User>> {
        let users = self
            .users
            .list_users(query.search.as_deref(), query.roles.as_deref())
            .await?;

        Ok(users)
    }

    pub async fn list_collaborator_candidates(
        &self,
        query: SearchUsersQuery,
    ) -> AppResult<Vec<User>> {
        let users = self
            .users
            .list_users(query.search.as_deref(), Some("assistant,func"))
            .await?;

        Ok(users)
    }

    pub async fn update_role(
        &self,
        current_user: &User,
        user_id: &Uuid,
        input: UpdateUserRoleRequest,
    ) -> AppResult<User> {
        let mut target = self
            .users
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| UsersError::NotFound(user_id.to_string()))?;

        self.policy
            .can_assign_assistant_role(current_user, &target)?;

        target.role = UserRole::from(input.role);

        self.users.save(&target).await
    }
}
