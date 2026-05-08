use crate::{
    attempts::AttemptError, authz::AuthzError, banks::QuestionBankError, courses::CoursesError,
    quizzes::QuizError, users::UsersError,
};

use jsonwebtoken::errors::Error as JwtError;
use ldap3::LdapError;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use sword::web::*;
use thiserror::Error;

pub type AppResult<T = JsonResponse> = Result<T, AppError>;

#[derive(Debug, Error, HttpError)]
pub enum AppError {
    #[http(code = 403)]
    #[tracing(error)]
    #[error("Unauthorized: {0}")]
    Unauthorized(#[from] JwtError),

    #[http(transparent)]
    #[error(transparent)]
    Quiz(#[from] QuizError),

    #[http(transparent)]
    #[error(transparent)]
    Users(#[from] UsersError),

    #[http(transparent)]
    #[error(transparent)]
    Courses(#[from] CoursesError),

    #[http(transparent)]
    #[error(transparent)]
    QuestionBank(#[from] QuestionBankError),

    #[http(transparent)]
    #[error(transparent)]
    Authz(#[from] AuthzError),

    #[http(transparent)]
    #[error(transparent)]
    Attempts(#[from] AttemptError),

    #[http(
        code = 401,
        message = "No se encontró el token solicitado. Intenta de nuevo o genera uno nuevo."
    )]
    #[error("Unauthorized access")]
    TokenNotFound,

    #[http(
        code = 401,
        message = "El token proporcionado no es válido. Intenta de nuevo o genera uno nuevo."
    )]
    #[error("Invalid token")]
    InvalidToken,

    #[http(code = 500)]
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[http(code = 500)]
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[http(code = 400)]
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[http(
        code = 401,
        message = "Credenciales inválidas. Inténtalo nuevamente o contacta a soporte."
    )]
    #[error("LDAP Authentication failed: {0}")]
    LdapAuth(#[from] LdapError),

    #[http(
        code = 401,
        message = "No hay un correo asociado a tu cuenta. Contacta a soporte."
    )]
    #[error("LDAP Email not found")]
    LdapEmailNotFound,

    #[http(
        code = 401,
        message = "No hay un nombre de usuario asociado a tu cuenta. Contacta a soporte."
    )]
    #[error("LDAP Error: {0}")]
    LdapUsernameNotFound(String),
}
