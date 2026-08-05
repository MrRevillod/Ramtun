use crate::banks::{Question, QuestionBankId};
use crate::courses::CourseId;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct QuestionBankView {
	pub id: QuestionBankId,
	pub course_id: CourseId,
	pub name: String,
	pub questions: Vec<Question>,
	pub created_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}
