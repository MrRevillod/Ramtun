use crate::shared::{Entity, Id};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Student,
    Func,
    Assistant,
}

pub type UserId = Id<User>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl Entity for User {
    fn key_name() -> &'static str {
        "user"
    }
}

impl User {
    pub fn new(username: String, name: String, email: String, role: UserRole) -> Self {
        Self {
            id: UserId::new(),
            username,
            name,
            email,
            role,
        }
    }
}
