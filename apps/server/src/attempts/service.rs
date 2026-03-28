use crate::attempts::*;
use crate::quizzes::*;
use crate::shared::AppResult;
use crate::users::User;

use chrono::{Duration, Utc};
use rand::seq::SliceRandom;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct AttemptService {
    attempts: AttemptRepository,
    policy: AttemptPolicy,
    quizzes: QuizRepository,
}

impl AttemptService {
    pub async fn start_attempt(
        &self,
        current_user: &User,
        quiz_id: &Uuid,
    ) -> AppResult<AttemptSnapshotView> {
        self.policy.can_start_attempt(current_user)?;

        let quiz = self.require_quiz(quiz_id).await?;

        if Utc::now() < quiz.start_time {
            return Err(AttemptError::NotStarted.into());
        }

        if let Some(attempt) = self
            .attempts
            .find_by_quiz_and_student(quiz_id, &current_user.id)
            .await?
        {
            if attempt.status == AttemptStatus::Submitted {
                return Err(AttemptError::AlreadySubmitted.into());
            }

            let answers = self.attempts.list_answers(&attempt.id).await?;
            return Ok(self.build_snapshot(&quiz, attempt, answers));
        }

        let question_order = shuffled_question_order(&quiz);
        let started_at = Utc::now();
        let expires_at = started_at + Duration::minutes(i64::from(quiz.attempt_duration_minutes));

        let attempt = AttemptEntity {
            id: Uuid::new_v4(),
            quiz_id: *quiz_id,
            student_id: current_user.id,
            started_at,
            expires_at,
            submitted_at: None,
            status: AttemptStatus::InProgress,
            question_order,
            created_at: started_at,
            updated_at: started_at,
        };

        let attempt = self.attempts.create(attempt).await?;

        Ok(self.build_snapshot(&quiz, attempt, Vec::new()))
    }

    pub async fn get_active_attempt_for_quiz(
        &self,
        current_user: &User,
        quiz_id: &Uuid,
    ) -> AppResult<AttemptSnapshotView> {
        self.policy.can_start_attempt(current_user)?;

        let Some(attempt) = self
            .attempts
            .find_active_for_quiz(quiz_id, &current_user.id)
            .await?
        else {
            return Err(AttemptError::ActiveAttemptNotFound(quiz_id.to_string()).into());
        };

        let quiz = self.require_quiz(&attempt.quiz_id).await?;
        let answers = self.attempts.list_answers(&attempt.id).await?;

        Ok(self.build_snapshot(&quiz, attempt, answers))
    }

    pub async fn save_answer(
        &self,
        current_user: &User,
        command: SaveAnswerCommand,
    ) -> AppResult<AttemptAnswerView> {
        let attempt = self
            .require_attempt_owner(current_user, &command.attempt_id)
            .await?;
        let quiz = self.require_quiz(&attempt.quiz_id).await?;
        self.policy.can_save_answer(current_user, &attempt)?;

        if !attempt.question_order.contains(&command.question_id) {
            return Err(AttemptError::QuestionNotInAttempt.into());
        }

        let Some(question) = quiz
            .questions
            .iter()
            .find(|question| question.id == command.question_id)
        else {
            return Err(AttemptError::QuestionNotInAttempt.into());
        };

        let is_invalid_answer = match usize::try_from(command.answer_index) {
            Ok(answer_index) => answer_index >= question.options.len(),
            Err(_) => true,
        };

        if is_invalid_answer {
            return Err(AttemptError::InvalidAnswerIndex.into());
        }

        let now = Utc::now();
        let answer = AttemptAnswerEntity {
            attempt_id: attempt.id,
            question_id: command.question_id,
            answer_index: command.answer_index,
            certainty_level: command.certainty_level,
            created_at: now,
            updated_at: now,
        };

        let answer = self.attempts.upsert_answer(answer).await?;

        Ok(AttemptAnswerView::from(answer))
    }

    pub async fn submit_attempt(
        &self,
        current_user: &User,
        attempt_id: &Uuid,
    ) -> AppResult<AttemptSnapshotView> {
        let attempt = self.require_attempt_owner(current_user, attempt_id).await?;
        self.policy.can_submit_attempt(current_user, &attempt)?;

        let attempt = self.attempts.submit(attempt_id).await?;
        let quiz = self.require_quiz(&attempt.quiz_id).await?;
        let answers = self.attempts.list_answers(attempt_id).await?;

        Ok(self.build_snapshot(&quiz, attempt, answers))
    }

    async fn require_quiz(&self, quiz_id: &Uuid) -> AppResult<QuizEntity> {
        let Some(quiz) = self.quizzes.find_by_id(quiz_id).await? else {
            return Err(AttemptError::NotFound(quiz_id.to_string()).into());
        };

        Ok(quiz)
    }

    async fn require_attempt_owner(
        &self,
        current_user: &User,
        attempt_id: &Uuid,
    ) -> AppResult<AttemptEntity> {
        let Some(attempt) = self.attempts.find_by_id(attempt_id).await? else {
            return Err(AttemptError::NotFound(attempt_id.to_string()).into());
        };

        self.policy.can_access_attempt(current_user, &attempt)?;

        Ok(attempt)
    }

    fn build_snapshot(
        &self,
        quiz: &QuizEntity,
        attempt: AttemptEntity,
        answers: Vec<AttemptAnswerEntity>,
    ) -> AttemptSnapshotView {
        let mut ordered_questions = Vec::with_capacity(attempt.question_order.len());

        for question_id in &attempt.question_order {
            if let Some(question) = quiz
                .questions
                .iter()
                .find(|question| question.id == *question_id)
            {
                ordered_questions.push(QuizQuestionView::from(question.clone()));
            }
        }

        let quiz_view = QuizParticipantView {
            id: quiz.id,
            title: quiz.title.clone(),
            kind: quiz.kind.clone(),
            questions: ordered_questions,
            certainly_table: quiz.certainly_table.clone(),
            start_time: quiz.start_time,
            attempt_duration_minutes: quiz.attempt_duration_minutes,
        };

        AttemptSnapshotView {
            attempt_id: attempt.id,
            quiz_id: attempt.quiz_id,
            started_at: attempt.started_at,
            expires_at: attempt.expires_at,
            status: attempt.status,
            quiz: quiz_view,
            answers: answers.into_iter().map(AttemptAnswerView::from).collect(),
        }
    }
}

fn shuffled_question_order(quiz: &QuizEntity) -> Vec<Uuid> {
    let mut question_order = quiz
        .questions
        .iter()
        .map(|question| question.id)
        .collect::<Vec<_>>();
    question_order.shuffle(&mut rand::rng());
    question_order
}
