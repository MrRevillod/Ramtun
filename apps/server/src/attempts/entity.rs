use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    banks::QuestionId,
    courses::CourseId,
    quizzes::{CertaintyLevel, QuizId},
    shared::{Entity, Id},
    users::UserId,
};

pub type AttemptId = Id<Attempt>;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, FromRow)]
pub struct Attempt {
    #[builder(default = AttemptId::new())]
    pub id: AttemptId,
    pub student_id: UserId,
    pub quiz_id: QuizId,
    pub question_order: Vec<QuestionId>,
    pub score: Option<i16>,
    pub grade: Option<f64>,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub results_viewed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, FromRow)]
pub struct AttemptAnswer {
    pub attempt_id: AttemptId,
    pub question_id: QuestionId,
    pub answer_index: i16,
    pub certainty_level: Option<CertaintyLevel>,
    pub is_correct: Option<bool>,
    pub awarded_points: Option<i16>,
}

impl Entity for Attempt {
    fn key_name() -> &'static str {
        "attempt"
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttemptFilter {
    pub course_id: CourseId,
    pub quiz_id: QuizId,
    pub student_id: Option<UserId>,
}
