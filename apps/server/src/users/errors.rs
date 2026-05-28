use sword::web::HttpError;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum UsersError {
    #[http(code = 404, message = "No se encontró el usuario solicitado.")]
    #[error("User not found: {0}")]
    NotFound(String),

    #[http(code = 400, message = "El rol de usuario proporcionado no es válido.")]
    #[error("Invalid user role provided")]
    InvalidUserRole,
}
