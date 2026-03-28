use crate::quizzes::{QuizEntity, QuizError, QuizRepository};
use crate::shared::AppResult;
use crate::users::{User, UserRole};

use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct QuizPolicy {
    repository: QuizRepository,
}

impl QuizPolicy {
    pub fn can_create_quiz(&self, current_user: &User) -> AppResult<()> {
        if Self::can_manage_quizzes(current_user) {
            return Ok(());
        }

        Err(QuizError::Forbidden.into())
    }

    pub async fn can_read_managed_quiz(
        &self,
        current_user: &User,
        quiz: &QuizEntity,
    ) -> AppResult<()> {
        if Self::has_managed_quiz_access(
            current_user,
            quiz,
            self.repository
                .is_collaborator(&quiz.id, &current_user.id)
                .await?,
        ) {
            return Ok(());
        }

        Err(QuizError::Forbidden.into())
    }

    pub async fn can_update_quiz(&self, current_user: &User, quiz: &QuizEntity) -> AppResult<()> {
        self.can_read_managed_quiz(current_user, quiz).await
    }

    pub fn can_join_quiz(&self, current_user: &User) -> AppResult<()> {
        if current_user.role == UserRole::Student {
            return Ok(());
        }

        Err(QuizError::Forbidden.into())
    }

    pub fn can_manage_collaborators(
        &self,
        current_user: &User,
        quiz: &QuizEntity,
    ) -> AppResult<()> {
        if Self::is_owner(current_user, quiz) {
            return Ok(());
        }

        Err(QuizError::OnlyOwnerCanManageCollaborators.into())
    }

    pub async fn can_list_managed_quizzes(&self, current_user: &User) -> AppResult<()> {
        if Self::can_manage_quizzes(current_user) {
            return Ok(());
        }

        Err(QuizError::Forbidden.into())
    }

    pub async fn require_managed_quiz(
        &self,
        current_user: &User,
        quiz_id: &Uuid,
    ) -> AppResult<QuizEntity> {
        let Some(quiz) = self.repository.find_by_id(quiz_id).await? else {
            return Err(QuizError::NotFound(quiz_id.to_string()).into());
        };

        self.can_read_managed_quiz(current_user, &quiz).await?;

        Ok(quiz)
    }

    pub async fn require_owner_quiz(
        &self,
        current_user: &User,
        quiz_id: &Uuid,
    ) -> AppResult<QuizEntity> {
        let Some(quiz) = self.repository.find_by_id(quiz_id).await? else {
            return Err(QuizError::NotFound(quiz_id.to_string()).into());
        };

        self.can_manage_collaborators(current_user, &quiz)?;

        Ok(quiz)
    }

    fn can_manage_quizzes(current_user: &User) -> bool {
        matches!(current_user.role, UserRole::Func | UserRole::Assistant)
    }

    fn is_owner(current_user: &User, quiz: &QuizEntity) -> bool {
        quiz.owner_id == current_user.id
    }

    fn has_managed_quiz_access(
        current_user: &User,
        quiz: &QuizEntity,
        is_collaborator: bool,
    ) -> bool {
        Self::is_owner(current_user, quiz) || is_collaborator
    }
}
