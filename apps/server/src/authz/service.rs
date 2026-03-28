use crate::{authz::AuthzAction, authz::AuthzError, shared::AppResult, users::UserRole};

use sword::prelude::*;

#[injectable]
pub struct AuthzService;

impl AuthzService {
    pub fn authorize_role(&self, role: &UserRole, action: AuthzAction) -> AppResult<()> {
        let allowed = match role {
            UserRole::Func => matches!(
                action,
                AuthzAction::CreateQuiz
                    | AuthzAction::ListManagedQuizzes
                    | AuthzAction::ReadManagedQuiz
                    | AuthzAction::UpdateManagedQuiz
                    | AuthzAction::ManageQuizCollaborators
                    | AuthzAction::JoinQuizByCode
                    | AuthzAction::StartAttempt
                    | AuthzAction::SaveOwnAttemptAnswer
                    | AuthzAction::SubmitOwnAttempt
                    | AuthzAction::ListUsersAdmin
                    | AuthzAction::ListCollaboratorCandidates
                    | AuthzAction::ManageAssistants
            ),
            UserRole::Assistant => matches!(
                action,
                AuthzAction::CreateQuiz
                    | AuthzAction::ListManagedQuizzes
                    | AuthzAction::ReadManagedQuiz
                    | AuthzAction::UpdateManagedQuiz
                    | AuthzAction::ListCollaboratorCandidates
            ),
            UserRole::Student => matches!(
                action,
                AuthzAction::JoinQuizByCode
                    | AuthzAction::StartAttempt
                    | AuthzAction::SaveOwnAttemptAnswer
                    | AuthzAction::SubmitOwnAttempt
            ),
        };

        if !allowed {
            return Err(AuthzError::Forbidden(action))?;
        }

        Ok(())
    }
}
