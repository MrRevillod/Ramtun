use sword::web::HttpError;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum UsersError {
    #[http(code = 403, message = "Solo profesores pueden asignar asistentes.")]
    #[error("Only professors can assign assistants")]
    OnlyProfessorsCanAssign,

    #[http(code = 404, message = "No se encontró el usuario solicitado.")]
    #[error("User not found: {0}")]
    NotFound(String),

    #[http(
        code = 400,
        message = "Solo estudiantes y asistentes pueden gestionarse desde este endpoint."
    )]
    #[error("Only students and assistants can be managed")]
    InvalidAssistantTargetRole,

    #[http(code = 400, message = "El rol de usuario proporcionado no es válido.")]
    #[error("Invalid user role provided")]
    InvalidUserRole,
}
