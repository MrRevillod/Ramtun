use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Student,
    Func,
    Assistant,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl User {
    pub fn new(username: String, name: String, email: String, role: UserRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            name,
            email,
            role,
        }
    }
}
