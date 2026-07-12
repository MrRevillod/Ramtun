use crate::{
	attempts::AttemptError, auth::AuthError, authz::AuthzError, banks::QuestionBankError,
	courses::CoursesError, quizzes::QuizError, users::UsersError,
};

use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use sword::web::*;
use thiserror::Error;

pub type AppResult<T = JsonResponse> = Result<T, AppError>;

#[derive(Debug, Error, HttpError)]
pub enum AppError {
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
	Auth(#[from] AuthError),

	#[http(transparent)]
	#[error(transparent)]
	Authz(#[from] AuthzError),

	#[http(transparent)]
	#[error(transparent)]
	Attempts(#[from] AttemptError),

	#[http(code = 500)]
	#[tracing(error)]
	#[error("Database error: {0}")]
	Database(#[from] SqlxError),

	#[http(code = 500)]
	#[tracing(error)]
	#[error("IO error: {0}")]
	Io(#[from] IoError),

	#[http(code = 400)]
	#[error("Bad request: {0}")]
	BadRequest(String),

	#[http(code = 500)]
	#[error("Internal Server error")]
	InternalError,
}
