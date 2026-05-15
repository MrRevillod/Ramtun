use std::sync::Arc;
use sword::prelude::*;
use sword::socketio::SocketIo;
use sword::web::*;

use crate::attempts::*;
use crate::auth::SessionCheck;
use crate::authz::*;
use crate::courses::CourseId;
use crate::quizzes::QuizId;
use crate::shared::RequestExt;

#[controller(kind = Controller::Web, path = "/attempts")]
#[interceptor(SessionCheck)]
pub struct AttemptsController {
    socket_io: SocketIo,
    attempts: Arc<AttemptsService>,
}

impl AttemptsController {
    #[get("/course/{courseId}/quiz/{quizId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::AttemptList)]
    async fn list_quiz_attempts(&self, req: Request) -> WebResult {
        let quiz_id = req.param::<QuizId>("quizId")?;
        let course_id = req.param::<CourseId>("courseId")?;

        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        let filter = AttemptFilter {
            course_id,
            quiz_id,
            student_id: None,
        };

        let attempts = self.attempts.list_attempts(user, filter).await?;

        Ok(JsonResponse::Ok().data(attempts))
    }

    #[post("/course/{courseId}/quiz/{quizId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::AttemptInitialize)]
    async fn initialize_attempt(&self, req: Request) -> WebResult {
        let quiz_id = req.param::<QuizId>("quizId")?;
        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        let attempt = self.attempts.initialize_attempt(quiz_id, user.id).await?;

        Ok(JsonResponse::Created().data(attempt))
    }

    #[put("/{attemptId}/answers/{questionId}")]
    #[interceptor(AuthzGuard, config = AuthzAction::AttemptSubmit)]
    async fn save_answer(&self, req: Request) -> WebResult {
        let input = req.body_validator::<SaveAttemptAnswerDto>()?;
        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        self.attempts.save_answer(user.id, input).await?;

        Ok(JsonResponse::Ok())
    }

    #[post("/{attemptId}/submit")]
    #[interceptor(AuthzGuard, config = AuthzAction::AttemptSubmit)]
    async fn submit_attempt(&self, req: Request) -> WebResult {
        let attempt_id = req.param::<AttemptId>("attemptId")?;
        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        let attempt = self.attempts.submit_attempt(attempt_id, user.id).await?;

        self.socket_io
            .broadcast()
            .emit("attempts:new-submit", &attempt)
            .await
            .ok();

        Ok(JsonResponse::Ok().data(attempt))
    }

    #[get("/join/{joinCode}/results/me")]
    #[interceptor(AuthzGuard, config = AuthzAction::QuizViewAttemptResultByCode)]
    async fn view_results_by_join_code(&self, req: Request) -> WebResult {
        let join_code = req.param::<String>("joinCode")?;
        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        let results = self
            .attempts
            .view_results_by_join_code(&join_code, user)
            .await?;

        Ok(JsonResponse::Ok().data(results))
    }

    #[get("/{attemptId}/results/managed")]
    #[interceptor(AuthzGuard, config = AuthzAction::AttemptViewResultsManaged)]
    async fn view_attempt_results_managed(&self, req: Request) -> WebResult {
        let attempt_id = req.param::<AttemptId>("attemptId")?;
        let user = req.user().ok_or(JsonResponse::Unauthorized())?;

        let results = self
            .attempts
            .view_attempt_results_managed(user, attempt_id)
            .await?;

        Ok(JsonResponse::Ok().data(results))
    }
}
