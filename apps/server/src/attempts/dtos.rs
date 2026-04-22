use crate::attempts::{AttemptCertaintyLevel, AttemptId};

use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_answer_request", skip_on_field_errors = false))]
pub struct SaveAnswerRequest {
    pub answer_index: i16,
    pub certainty_level: Option<AttemptCertaintyLevel>,
}

fn validate_answer_request(request: &SaveAnswerRequest) -> Result<(), ValidationError> {
    if request.answer_index < 0 {
        let mut err = ValidationError::new("invalid_answer_index");
        err.message = Some("answerIndex must be zero or greater".into());
        return Err(err);
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct SaveAnswerCommand {
    pub attempt_id: AttemptId,
    pub question_id: Uuid,
    pub answer_index: i16,
    pub certainty_level: Option<AttemptCertaintyLevel>,
}
