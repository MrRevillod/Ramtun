use crate::attempts::{AttemptEntity, AttemptError, AttemptStatus};
use crate::shared::AppResult;
use crate::users::{User, UserRole};

use chrono::Utc;
use sword::prelude::*;

#[injectable]
pub struct AttemptPolicy;

impl AttemptPolicy {
    pub fn can_start_attempt(&self, current_user: &User) -> AppResult<()> {
        if current_user.role == UserRole::Student {
            return Ok(());
        }

        Err(AttemptError::Forbidden.into())
    }

    pub fn can_access_attempt(
        &self,
        current_user: &User,
        attempt: &AttemptEntity,
    ) -> AppResult<()> {
        if attempt.student_id == current_user.id {
            return Ok(());
        }

        Err(AttemptError::Forbidden.into())
    }

    pub fn can_submit_attempt(
        &self,
        current_user: &User,
        attempt: &AttemptEntity,
    ) -> AppResult<()> {
        self.can_access_attempt(current_user, attempt)?;

        if attempt.status == AttemptStatus::Submitted {
            return Err(AttemptError::AlreadySubmitted.into());
        }

        if Utc::now() > attempt.expires_at {
            return Err(AttemptError::Expired.into());
        }

        Ok(())
    }

    pub fn can_save_answer(&self, current_user: &User, attempt: &AttemptEntity) -> AppResult<()> {
        self.can_submit_attempt(current_user, attempt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use uuid::Uuid;

    fn user(user_id: Uuid, role: UserRole) -> User {
        User {
            id: user_id,
            username: "user".to_string(),
            name: "User".to_string(),
            email: "user@example.com".to_string(),
            role,
        }
    }

    fn attempt(
        student_id: Uuid,
        status: AttemptStatus,
        expires_at: chrono::DateTime<Utc>,
    ) -> AttemptEntity {
        AttemptEntity {
            id: Uuid::new_v4(),
            quiz_id: Uuid::new_v4(),
            student_id,
            started_at: Utc::now(),
            expires_at,
            submitted_at: (status == AttemptStatus::Submitted).then(Utc::now),
            status,
            question_order: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn student_can_start_attempt() {
        let policy = AttemptPolicy;
        let current_user = user(Uuid::new_v4(), UserRole::Student);

        assert!(policy.can_start_attempt(&current_user).is_ok());
    }

    #[test]
    fn assistant_cannot_start_attempt() {
        let policy = AttemptPolicy;
        let current_user = user(Uuid::new_v4(), UserRole::Assistant);

        assert!(policy.can_start_attempt(&current_user).is_err());
    }

    #[test]
    fn student_can_access_own_attempt() {
        let policy = AttemptPolicy;
        let student_id = Uuid::new_v4();
        let current_user = user(student_id, UserRole::Student);
        let attempt = attempt(
            student_id,
            AttemptStatus::InProgress,
            Utc::now() + Duration::minutes(5),
        );

        assert!(policy.can_access_attempt(&current_user, &attempt).is_ok());
    }

    #[test]
    fn student_cannot_access_other_attempt() {
        let policy = AttemptPolicy;
        let current_user = user(Uuid::new_v4(), UserRole::Student);
        let attempt = attempt(
            Uuid::new_v4(),
            AttemptStatus::InProgress,
            Utc::now() + Duration::minutes(5),
        );

        assert!(policy.can_access_attempt(&current_user, &attempt).is_err());
    }

    #[test]
    fn cannot_submit_submitted_attempt() {
        let policy = AttemptPolicy;
        let student_id = Uuid::new_v4();
        let current_user = user(student_id, UserRole::Student);
        let attempt = attempt(
            student_id,
            AttemptStatus::Submitted,
            Utc::now() + Duration::minutes(5),
        );

        assert!(matches!(
            policy.can_submit_attempt(&current_user, &attempt),
            Err(crate::shared::AppError::Attempt(
                AttemptError::AlreadySubmitted
            ))
        ));
    }

    #[test]
    fn cannot_submit_expired_attempt() {
        let policy = AttemptPolicy;
        let student_id = Uuid::new_v4();
        let current_user = user(student_id, UserRole::Student);
        let attempt = attempt(
            student_id,
            AttemptStatus::InProgress,
            Utc::now() - Duration::minutes(1),
        );

        assert!(matches!(
            policy.can_submit_attempt(&current_user, &attempt),
            Err(crate::shared::AppError::Attempt(AttemptError::Expired))
        ));
    }
}
