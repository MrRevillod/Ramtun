use crate::attempts::{AttemptError, AttemptService, SaveAnswerCommand, SaveAnswerRequest};
use crate::auth::SessionCheck;
use crate::authz::{AuthzAction, AuthzGuard};
use crate::users::User;

use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[controller("/attempts")]
#[interceptor(SessionCheck)]
pub struct AttemptController {
    service: Arc<AttemptService>,
}

impl AttemptController {
    #[put("/{attemptId}/answers/{questionId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::SaveOwnAttemptAnswer)]
    pub async fn save_answer(&self, req: Request) -> HttpResult {
        let attempt_id = req
            .param::<Uuid>("attemptId")
            .map_err(|_| AttemptError::InvalidAttemptId)?;
        let question_id = req
            .param::<Uuid>("questionId")
            .map_err(|_| AttemptError::InvalidQuestionId)?;
        let current_user = extract_current_user(&req)?;
        let input = req.body_validator::<SaveAnswerRequest>()?;

        let answer = self
            .service
            .save_answer(
                &current_user,
                SaveAnswerCommand {
                    attempt_id,
                    question_id,
                    answer_index: input.answer_index,
                    certainty_level: input.certainty_level,
                },
            )
            .await?;

        Ok(JsonResponse::Ok().data(answer))
    }

    #[post("/{attemptId}/submit")]
    #[interceptor(AuthzGuard, config = AuthzAction::SubmitOwnAttempt)]
    pub async fn submit(&self, req: Request) -> HttpResult {
        let attempt_id = req
            .param::<Uuid>("attemptId")
            .map_err(|_| AttemptError::InvalidAttemptId)?;
        let current_user = extract_current_user(&req)?;

        let attempt = self
            .service
            .submit_attempt(&current_user, &attempt_id)
            .await?;

        Ok(JsonResponse::Ok().data(attempt))
    }
}

fn extract_current_user(req: &Request) -> Result<User, JsonResponse> {
    req.extensions
        .get::<User>()
        .cloned()
        .ok_or_else(JsonResponse::Unauthorized)
}
