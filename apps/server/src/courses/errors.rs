use sword::web::HttpError;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum CoursesError {
    #[http(code = 404, message = "No se encontró el curso solicitado.")]
    #[error("Course not found: {0}")]
    NotFound(String),

    #[http(code = 400, message = "El ID del curso proporcionado no es válido.")]
    #[error("Invalid course ID")]
    InvalidId,

    #[http(code = 403, message = "No tienes acceso a este curso.")]
    #[error("Forbidden course access")]
    Forbidden,

    #[http(
        code = 403,
        message = "Solo asistentes o docentes del curso pueden gestionar miembros."
    )]
    #[error("Only assistant or func members can manage course members")]
    OnlyFuncCanManageMembers,

    #[http(code = 409, message = "Ya existe un curso con este código.")]
    #[error("Course code already exists")]
    CodeAlreadyExists,

    #[http(code = 409, message = "Este usuario ya es miembro de este curso.")]
    #[error("Course member already exists")]
    MemberAlreadyExists,

    #[http(
        code = 404,
        message = "No se encontró el miembro del curso solicitado."
    )]
    #[error("Course member not found")]
    MemberNotFound,

    #[http(
        code = 400,
        message = "Solo usuarios asistentes o docentes pueden ser miembros del curso."
    )]
    #[error("Invalid course member role")]
    InvalidMemberRole,

    #[http(
        code = 409,
        message = "No puedes eliminar al último docente de un curso."
    )]
    #[error("Cannot remove last func member")]
    CannotRemoveLastFuncMember,

    #[http(
        code = 409,
        message = "No puedes eliminarte a ti mismo si eres el último miembro de un curso. Puedes eliminar el curso."
    )]
    #[error("Cannot remove last member")]
    CannotRemoveLastMember,
}
