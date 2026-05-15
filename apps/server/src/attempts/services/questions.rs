use rand::seq::SliceRandom;
use std::{collections::HashMap, sync::Arc};
use sword::prelude::*;
use uuid::Uuid;

use crate::{
    attempts::{AttemptError, AttemptRepository},
    banks::{Question, QuestionId, QuestionView},
    quizzes::{QuizError, QuizId},
    shared::AppResult,
    snapshots::SnapshotRepository,
};

#[injectable]
pub struct QuestionService {
    snapshots: Arc<SnapshotRepository>,
    repository: Arc<AttemptRepository>,
}

impl QuestionService {
    /// Retrieves all questions associated with a specific quiz.
    pub async fn get_quiz_questions(&self, quiz_id: &QuizId) -> AppResult<Vec<Question>> {
        self.snapshots.list_questions_for_quiz(quiz_id).await
    }

    /// Retrieves a specific question by its ID for a given quiz.
    pub async fn get_question_by_id(
        &self,
        quiz_id: &QuizId,
        question_id: &Uuid,
    ) -> AppResult<Option<Question>> {
        self.snapshots
            .find_question_for_quiz(quiz_id, question_id)
            .await
    }

    /// Filters a list of questions based on a provided list of question IDs,
    /// ensuring the order of the returned questions matches the order of the IDs.
    pub async fn filter_questions(
        &self,
        questions: Vec<Question>,
        question_ids: &[QuestionId],
    ) -> AppResult<Vec<Question>> {
        let mut ordered = Vec::with_capacity(question_ids.len());

        for question_id in question_ids {
            let Some(question) = questions.iter().find(|q| &q.id == question_id) else {
                continue;
            };

            ordered.push(question.clone());
        }

        Ok(ordered)
    }

    /// Converts a list of questions into a HashMap
    /// where the key is the question ID and the value is the question itself.
    pub async fn into_map(&self, qs: Vec<Question>) -> AppResult<HashMap<QuestionId, Question>> {
        let q = qs
            .into_iter()
            .map(|q| (q.id, q))
            .collect::<HashMap<QuestionId, Question>>();

        Ok(q)
    }

    /// Converts a list of questions into a list of question views.
    pub async fn into_views(&self, qs: Vec<Question>) -> AppResult<Vec<QuestionView>> {
        Ok(qs.into_iter().map(QuestionView::from).collect::<Vec<_>>())
    }

    /// Selects a random subset of question IDs for a quiz attempt, based on the specified number of questions.
    /// Also shuffle the questions to ensure a different order for each attempt.
    pub async fn get_question_ids_for_attempt(
        &self,
        quiz_id: &QuizId,
        question_number: usize,
    ) -> AppResult<Vec<QuestionId>> {
        let questions = self.snapshots.list_questions_for_quiz(quiz_id).await?;

        if questions.len() < question_number {
            return Err(QuizError::InvalidQuestionCount)?;
        }

        let mut question_ids = questions.into_iter().map(|q| q.id).collect::<Vec<_>>();

        question_ids.shuffle(&mut rand::rng());

        let selected_ids = question_ids
            .into_iter()
            .take(question_number)
            .collect::<Vec<_>>();

        Ok(selected_ids)
    }

    /// Ensures that a given question ID is part of the current attempt's question order.
    pub fn ensure_question_belongs_to_attempt(
        &self,
        question_order: &[QuestionId],
        question_id: &QuestionId,
    ) -> AppResult<()> {
        if !question_order.contains(question_id) {
            return Err(AttemptError::QuestionNotInAttempt(*question_id))?;
        }

        Ok(())
    }

    /// Validates that a provided answer index is within the valid range of options for a given question.
    pub fn ensure_valid_answer_index(
        &self,
        question: &Question,
        answer_index: i16,
    ) -> AppResult<()> {
        let max = question.options.len() as i16;

        if answer_index < 0 || answer_index >= max {
            return Err(AttemptError::InvalidAnswerIndex)?;
        }

        Ok(())
    }
}
