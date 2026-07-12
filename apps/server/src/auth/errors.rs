use jsonwebtoken::errors::Error as JwtError;
use ldap3::LdapError;
use sword::web::HttpError;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
#[http_error(code = 401, message = "Unauthorized")]
pub enum AuthError {
    #[error("The requested token was not found")]
    TokenNotFound,

    #[error("The provided token is invalid")]
    InvalidToken,

    #[http(
        code = 401,
        message = "Credenciales inválidas. Verifica tu usuario y contraseña."
    )]
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("LDAP authentication failed: {0}")]
    Ldap(#[from] LdapError),

    #[http(
        code = 401,
        message = "No hay un correo asociado a tu cuenta. Contacta a soporte."
    )]
    #[error("LDAP email not found")]
    LdapEmailNotFound,

    #[http(
        code = 401,
        message = "Credenciales inválidas. Verifica tu usuario y contraseña."
    )]
    #[error("LDAP username not found: {0}")]
    LdapUsernameNotFound(String),

    #[http(code = 403)]
    #[tracing(error)]
    #[error("JWT Error: {0}")]
    Jwt(#[from] JwtError),

    #[http(code = 500, message = "Internal server error")]
    #[tracing(error)]
    #[error("Internal server error - Hashing: {0}")]
    Hashing(#[from] Box<dyn std::error::Error + Send + Sync>),
}
