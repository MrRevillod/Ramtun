use crate::attempts::{AttemptAnswerEntity, AttemptEntity};
use crate::shared::{AppResult, Database};

use chrono::Utc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct AttemptRepository {
    db: Database,
}

impl AttemptRepository {
    pub async fn find_by_id(&self, attempt_id: &Uuid) -> AppResult<Option<AttemptEntity>> {
        let attempt =
            sqlx::query_as::<_, AttemptEntity>("SELECT * FROM quiz_attempts WHERE id = $1")
                .bind(attempt_id)
                .fetch_optional(self.db.get_pool())
                .await?;

        Ok(attempt)
    }

    pub async fn find_active_for_quiz(
        &self,
        quiz_id: &Uuid,
        student_id: &Uuid,
    ) -> AppResult<Option<AttemptEntity>> {
        let attempt = sqlx::query_as::<_, AttemptEntity>(
            "SELECT *
             FROM quiz_attempts
             WHERE quiz_id = $1 AND student_id = $2 AND status = 'in_progress'",
        )
        .bind(quiz_id)
        .bind(student_id)
        .fetch_optional(self.db.get_pool())
        .await?;

        Ok(attempt)
    }

    pub async fn find_by_quiz_and_student(
        &self,
        quiz_id: &Uuid,
        student_id: &Uuid,
    ) -> AppResult<Option<AttemptEntity>> {
        let attempt = sqlx::query_as::<_, AttemptEntity>(
            "SELECT *
             FROM quiz_attempts
             WHERE quiz_id = $1 AND student_id = $2",
        )
        .bind(quiz_id)
        .bind(student_id)
        .fetch_optional(self.db.get_pool())
        .await?;

        Ok(attempt)
    }

    pub async fn create(&self, attempt: AttemptEntity) -> AppResult<AttemptEntity> {
        let attempt = sqlx::query_as::<_, AttemptEntity>(
            "INSERT INTO quiz_attempts (
                id,
                quiz_id,
                student_id,
                started_at,
                expires_at,
                submitted_at,
                status,
                question_order,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *",
        )
        .bind(&attempt.id)
        .bind(&attempt.quiz_id)
        .bind(&attempt.student_id)
        .bind(&attempt.started_at)
        .bind(&attempt.expires_at)
        .bind(&attempt.submitted_at)
        .bind(&attempt.status)
        .bind(&attempt.question_order)
        .bind(&attempt.created_at)
        .bind(&attempt.updated_at)
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(attempt)
    }

    pub async fn list_answers(&self, attempt_id: &Uuid) -> AppResult<Vec<AttemptAnswerEntity>> {
        let answers = sqlx::query_as::<_, AttemptAnswerEntity>(
            "SELECT * FROM quiz_answers WHERE attempt_id = $1 ORDER BY created_at ASC",
        )
        .bind(attempt_id)
        .fetch_all(self.db.get_pool())
        .await?;

        Ok(answers)
    }

    pub async fn upsert_answer(
        &self,
        answer: AttemptAnswerEntity,
    ) -> AppResult<AttemptAnswerEntity> {
        let answer = sqlx::query_as::<_, AttemptAnswerEntity>(
            "INSERT INTO quiz_answers (
                attempt_id,
                question_id,
                answer_index,
                certainty_level,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (attempt_id, question_id) DO UPDATE
            SET answer_index = EXCLUDED.answer_index,
                certainty_level = EXCLUDED.certainty_level,
                updated_at = EXCLUDED.updated_at
            RETURNING *",
        )
        .bind(&answer.attempt_id)
        .bind(&answer.question_id)
        .bind(&answer.answer_index)
        .bind(&answer.certainty_level)
        .bind(&answer.created_at)
        .bind(&answer.updated_at)
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(answer)
    }

    pub async fn submit(&self, attempt_id: &Uuid) -> AppResult<AttemptEntity> {
        let now = Utc::now();

        let attempt = sqlx::query_as::<_, AttemptEntity>(
            "UPDATE quiz_attempts
             SET submitted_at = $2,
                 status = 'submitted',
                 updated_at = $2
             WHERE id = $1
             RETURNING *",
        )
        .bind(attempt_id)
        .bind(now)
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(attempt)
    }
}
