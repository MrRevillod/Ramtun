use chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;

use crate::{
    attempts::*,
    quizzes::{QuizId, QuizRepository},
    shared::AppResult,
};

#[injectable]
pub struct WarningService {
    repository: Arc<AttemptRepository>,
    quizzes: Arc<QuizRepository>,
}

impl WarningService {
    pub async fn record_warning(
        &self,
        attempt_id: AttemptId,
        warning_type: WarningType,
        details: &str,
    ) -> AppResult<AttemptWarning> {
        let attempt = self
            .repository
            .find_by_id(&attempt_id)
            .await?
            .ok_or(AttemptError::NotFound(attempt_id))?;

        if attempt.submitted_at.is_some() {
            Err(AttemptError::AlreadySubmitted)?;
        }

        let sequence_number = self
            .repository
            .get_next_warning_sequence(&attempt_id)
            .await?;

        let warning = AttemptWarning {
            id: AttemptWarningId::new(),
            attempt_id,
            warning_type,
            details: details.to_string(),
            sequence_number,
            created_at: Utc::now(),
        };

        self.repository.insert_warning(&warning).await?;

        Ok(warning)
    }

    pub async fn get_warnings_for_attempt(
        &self,
        attempt_id: AttemptId,
    ) -> AppResult<Vec<AttemptWarning>> {
        self.repository.get_warnings_by_attempt(&attempt_id).await
    }

    pub async fn get_warnings_for_quiz(&self, quiz_id: QuizId) -> AppResult<Vec<AttemptWarning>> {
        self.repository.get_warnings_by_quiz(&quiz_id).await
    }
}
