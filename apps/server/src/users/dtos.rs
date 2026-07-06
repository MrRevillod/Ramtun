use crate::users::{User, UserFilter, UserId, UserRole, UsersError};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::str::FromStr;
use validator::Validate;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManageableUserRole {
    Student,
    Func,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleDto {
    pub role: ManageableUserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePasswordDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchUsersQuery {
    pub search: Option<String>,
    pub roles: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserView {
    pub id: UserId,
    pub username: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}

impl From<ManageableUserRole> for UserRole {
    fn from(value: ManageableUserRole) -> Self {
        match value {
            ManageableUserRole::Student => Self::Student,
            ManageableUserRole::Func => Self::Func,
        }
    }
}

impl FromStr for UserRole {
    type Err = UsersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "student" => Ok(Self::Student),
            "func" => Ok(Self::Func),
            "admin" => Ok(Self::Admin),
            _ => Err(UsersError::InvalidUserRole),
        }
    }
}

impl From<SearchUsersQuery> for UserFilter {
    fn from(value: SearchUsersQuery) -> Self {
        let roles = value.roles.as_ref().map(|roles| {
            roles
                .split(',')
                .filter_map(|r| UserRole::from_str(r.trim()).ok())
                .collect::<Vec<_>>()
        });

        Self {
            search: value.search,
            roles,
        }
    }
}
