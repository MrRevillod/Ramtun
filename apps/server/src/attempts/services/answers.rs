use crate::{
    attempts::{AttemptAnswer, AttemptId, AttemptRepository},
    banks::QuestionId,
    shared::AppResult,
};

use std::{collections::HashMap, sync::Arc};
use sword::prelude::*;

#[injectable]
pub struct AnswerService {
    repository: Arc<AttemptRepository>,
}

impl AnswerService {
    pub async fn get_attempt_answers(
        &self,
        attempt_id: &AttemptId,
    ) -> AppResult<Vec<AttemptAnswer>> {
        self.repository.list_attempt_answers(attempt_id).await
    }

    /// Converts a list of AttemptAnswer into a HashMap
    /// where the key is the question_id and the value is the AttemptAnswer.
    pub async fn into_map(
        &self,
        answers: Vec<AttemptAnswer>,
    ) -> AppResult<HashMap<QuestionId, AttemptAnswer>> {
        let map = answers
            .into_iter()
            .map(|answer| (answer.question_id, answer))
            .collect();

        Ok(map)
    }
}
