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
        message = "Tu cuenta no tiene permisos para acceder a esta aplicación. Contacta a soporte."
    )]
    #[error("LDAP username not found: {0}")]
    LdapUsernameNotFound(String),

    #[http(code = 403)]
    #[tracing(error)]
    #[error("JWT Error: {0}")]
    Jwt(#[from] JwtError),
}
