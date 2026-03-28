use crate::users::UserRole;
use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManageableUserRole {
    Student,
    Assistant,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleRequest {
    pub role: ManageableUserRole,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchUsersQuery {
    pub search: Option<String>,
    pub roles: Option<String>,
}

impl From<ManageableUserRole> for UserRole {
    fn from(value: ManageableUserRole) -> Self {
        match value {
            ManageableUserRole::Student => Self::Student,
            ManageableUserRole::Assistant => Self::Assistant,
        }
    }
}
