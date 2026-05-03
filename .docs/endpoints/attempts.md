# Attempts Endpoints

## GET `/attempts/course/{courseId}/quiz/{quizId}`
- Summary: List attempts for a quiz (management view).
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptList): `admin`, `func`, `assistant`, `student`.
- Policy: currently requires course membership as `func` in service policy.
- Path params:
  - `courseId: uuid`
  - `quizId: uuid`
- Response `data`: `AttemptListItemView[]`
  - `{ attemptId, studentId, quizId, startedAt, expiresAt, submittedAt, resultsViewedAt, score, grade }`

## POST `/attempts/course/{courseId}/quiz/{quizId}`
- Summary: Initialize an attempt for current user.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptInitialize): `admin`, `func`, `assistant`, `student`.
- Policy: quiz must exist, started, not closed, and user must not have a previous attempt.
- Path params:
  - `courseId: uuid`
  - `quizId: uuid`
- Body: none.
- Response `data`: `AttemptView`
  - `{ attemptId, quizId, startedAt, expiresAt, submittedAt, resultsViewedAt, questions }`
  - `questions[]`: `{ id, prompt, options, images }`

## PUT `/attempts/{attemptId}/answers/{questionId}`
- Summary: Save or update one answer in attempt.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptSubmit): `admin`, `func`, `assistant`, `student`.
- Policy: attempt must belong to current user, be open, not submitted, and question must belong to attempt.
- Path params:
  - `attemptId: uuid`
  - `questionId: uuid`
- Body:

```json
{
  "answerIndex": 1,
  "certaintyLevel": "low"
}
```

- Notes:
  - `answerIndex >= 0`.
  - `certaintyLevel` required for `certainty`, forbidden for `traditional`.
- Response `data`: none (success envelope only).

## POST `/attempts/{attemptId}/submit`
- Summary: Submit an attempt and persist score/grade.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptSubmit): `admin`, `func`, `assistant`, `student`.
- Policy: attempt must belong to current user, not expired, and not already submitted.
- Path params:
  - `attemptId: uuid`
- Body: none.
- Response `data`: `AttemptSubmitView`
  - `{ attemptId, quizId, submittedAt }`

## GET `/attempts/{attemptId}/results`
- Summary: View evaluated attempt result.
- Auth: Bearer token required.
- Roles (AuthzAction::AttemptViewResults): `admin`, `func`, `assistant`, `student`.
- Policy: attempt must belong to current user and must be submitted.
- Path params:
  - `attemptId: uuid`
- Response `data`: `AttemptResultView`
  - `{ attemptId, quizId, submittedAt, grade, score, maxScore, resultsViewedAt, questions }`
  - `questions[]`: `{ questionId, question, options, images, answerIndex, correctAnswerIndex, certaintyLevel, isCorrect, awardedPoints }`

## Common Domain Errors
- `403`: quiz not started, quiz ended, expired, already submitted, forbidden.
- `404`: attempt not found.
- `400`: invalid answer index, certainty rule violation, question not in attempt.
