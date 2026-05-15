use serde::Deserialize;
use validator::Validate;

use crate::{attempts::AttemptId, banks::QuestionId, quizzes::CertaintyLevel};

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaveAttemptAnswerDto {
    #[validate(range(min = 0, message = "answerIndex must be greater or equal than 0"))]
    pub answer_index: i16,
    pub certainty_level: Option<CertaintyLevel>,

    pub question_id: QuestionId,
    pub attempt_id: AttemptId,
}
