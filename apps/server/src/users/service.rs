use crate::shared::{AppResult, Hasher};
use crate::users::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersService {
    policy: Arc<UserPolicy>,
    users: Arc<UserRepository>,
    hasher: Arc<Hasher>,
}

impl UsersService {
    pub async fn list_users(&self, query: SearchUsersQuery) -> AppResult<Vec<UserView>> {
        self.users.list_users(UserFilter::from(query)).await
    }

    pub async fn find_by_id(&self, user_id: &UserId) -> AppResult<UserView> {
        let Some(user) = self.users.find_by_id(user_id).await? else {
            return Err(UsersError::NotFound(*user_id))?;
        };

        Ok(user.into())
    }

    pub async fn list_collaborator_candidates(
        &self,
        query: SearchUsersQuery,
    ) -> AppResult<Vec<UserView>> {
        let filter = UserFilter::from(query);

        if let Some(roles) = &filter.roles
            && roles.contains(&UserRole::Student)
        {
            return Err(UsersError::InvalidUserRole)?;
        }

        let mut candidates = self.users.list_users(filter).await?;
        candidates.retain(|u| u.role != UserRole::Admin);

        Ok(candidates)
    }

    pub async fn update_role(&self, user_id: &UserId, input: UpdateUserRoleDto) -> AppResult<()> {
        let mut target = self
            .users
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| UsersError::NotFound(*user_id))?;

        target.role = UserRole::from(input.role);

        self.users.save(&target).await?;

        Ok(())
    }

    pub async fn update_password(&self, input: UpdatePasswordDto) -> AppResult<()> {
        let mut target = self
            .users
            .find_by_email(&input.email)
            .await?
            .ok_or_else(|| UsersError::UserNotFound)?;

        target.password_hash = self.hasher.hash(&input.password).await?;

        self.users.save(&target).await?;

        Ok(())
    }
}
