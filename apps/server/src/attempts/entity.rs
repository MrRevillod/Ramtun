use crate::quizzes::QuizParticipantView;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "attempt_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    InProgress,
    Submitted,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "attempt_certainty_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AttemptCertaintyLevel {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AttemptEntity {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub student_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub status: AttemptStatus,
    pub question_order: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AttemptAnswerEntity {
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_index: i16,
    pub certainty_level: Option<AttemptCertaintyLevel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptAnswerView {
    pub question_id: Uuid,
    pub answer_index: i16,
    pub certainty_level: Option<AttemptCertaintyLevel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptSnapshotView {
    pub attempt_id: Uuid,
    pub quiz_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: AttemptStatus,
    pub quiz: QuizParticipantView,
    pub answers: Vec<AttemptAnswerView>,
}

impl From<AttemptAnswerEntity> for AttemptAnswerView {
    fn from(answer: AttemptAnswerEntity) -> Self {
        Self {
            question_id: answer.question_id,
            answer_index: answer.answer_index,
            certainty_level: answer.certainty_level,
        }
    }
}
