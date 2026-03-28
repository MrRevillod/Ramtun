use crate::attempts::AttemptService;
use crate::auth::SessionCheck;
use crate::authz::{AuthzAction, AuthzGuard};
use crate::quizzes::*;
use crate::users::User;

use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[controller("/quizzes")]
#[interceptor(SessionCheck)]
pub struct QuizController {
    service: Arc<QuizService>,
    attempts: Arc<AttemptService>,
}

impl QuizController {
    #[get("/{quizId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::ReadManagedQuiz)]
    pub async fn get_detail(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        let quiz = self.service.get_detail(&current_user, &quiz_id).await?;

        Ok(JsonResponse::Ok().data(quiz))
    }

    #[get("/me")]
    #[interceptor(AuthzGuard, config = AuthzAction::ListManagedQuizzes)]
    pub async fn list_managed(&self, req: Request) -> HttpResult {
        let current_user = extract_current_user(&req)?;
        let quizzes = self.service.list_managed_by_user(&current_user).await?;

        Ok(JsonResponse::Ok().data(quizzes))
    }

    #[post("/")]
    #[interceptor(AuthzGuard, config = AuthzAction::CreateQuiz)]
    pub async fn create(&self, req: Request) -> HttpResult {
        let current_user = extract_current_user(&req)?;
        let input = req.body_validator::<CreateQuizRequest>()?;
        let quiz = self
            .service
            .create(&current_user, input, current_user.id)
            .await?;

        Ok(JsonResponse::Created().data(quiz))
    }

    #[patch("/{quizId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::UpdateManagedQuiz)]
    pub async fn update(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;
        let input = req.body_validator::<UpdateQuizRequest>()?;
        let quiz = self.service.update(&current_user, &quiz_id, input).await?;

        Ok(JsonResponse::Ok().data(quiz))
    }

    #[post("/join-by-code")]
    #[interceptor(AuthzGuard, config = AuthzAction::JoinQuizByCode)]
    pub async fn join_by_code(&self, req: Request) -> HttpResult {
        let current_user = extract_current_user(&req)?;
        let input = req.body_validator::<JoinQuizByCodeRequest>()?;
        let preview = self
            .service
            .get_join_preview(&current_user, &input.code)
            .await?;

        Ok(JsonResponse::Ok().data(preview))
    }

    #[post("/{quizId}/attempts")]
    #[interceptor(AuthzGuard, config = AuthzAction::StartAttempt)]
    pub async fn start_attempt(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        let attempt = self.attempts.start_attempt(&current_user, &quiz_id).await?;

        Ok(JsonResponse::Created().data(attempt))
    }

    #[get("/{quizId}/attempts/me")]
    #[interceptor(AuthzGuard, config = AuthzAction::StartAttempt)]
    pub async fn get_active_attempt(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        let attempt = self
            .attempts
            .get_active_attempt_for_quiz(&current_user, &quiz_id)
            .await?;

        Ok(JsonResponse::Ok().data(attempt))
    }

    #[put("/{quizId}/collaborators/{userId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::ManageQuizCollaborators)]
    pub async fn add_collaborator(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let user_id = req
            .param::<Uuid>("userId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        self.service
            .add_collaborator(&current_user, &quiz_id, AddCollaboratorRequest { user_id })
            .await?;

        Ok(JsonResponse::Ok().message("Collaborator added successfully"))
    }

    #[delete("/{quizId}/collaborators/{userId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::ManageQuizCollaborators)]
    pub async fn remove_collaborator(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let user_id = req
            .param::<Uuid>("userId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        self.service
            .remove_collaborator(&current_user, &quiz_id, &user_id)
            .await?;

        Ok(JsonResponse::Ok().message("Collaborator removed successfully"))
    }

    #[get("/{quizId}/collaborators")]
    #[interceptor(AuthzGuard, config = AuthzAction::ReadManagedQuiz)]
    pub async fn list_collaborators(&self, req: Request) -> HttpResult {
        let quiz_id = req
            .param::<Uuid>("quizId")
            .map_err(|_| QuizError::InvalidId)?;
        let current_user = extract_current_user(&req)?;

        let collaborators = self
            .service
            .list_collaborators(&current_user, &quiz_id)
            .await?;

        Ok(JsonResponse::Ok().data(collaborators))
    }
}

fn extract_current_user(req: &Request) -> Result<User, JsonResponse> {
    req.extensions
        .get::<User>()
        .cloned()
        .ok_or_else(JsonResponse::Unauthorized)
}
