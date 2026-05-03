# Quizzes Endpoints

## GET `/quizzes/{quizId}`
- Summary: Get one managed quiz.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizReadManaged): `admin`, `func`, `assistant`.
- Policy: user must have access to the quiz context.
- Path params:
  - `quizId: uuid`
- Response `data`: `QuizView`

## GET `/quizzes/me`
- Summary: List quizzes managed by current user.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizListManaged): `admin`, `func`, `assistant`.
- Response `data`: `QuizView[]`

## POST `/quizzes`
- Summary: Create quiz from course + selected banks.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizCreate): `admin`, `func`, `assistant`.
- Policy: user must be allowed in course and bank context.
- Body:

```json
{
  "courseId": "uuid",
  "title": "Control Semana 3",
  "kind": "traditional",
  "startsAt": "2026-05-10T15:00:00Z",
  "attemptDurationMinutes": 30,
  "questionCount": 10,
  "bankIds": ["uuid"],
  "certaintyConfig": null
}
```

- Notes:
  - `kind=certainty` requires `certaintyConfig`.
  - `kind=traditional` must not include `certaintyConfig`.
- Response `data`: `QuizView`

## DELETE `/quizzes/{quizId}`
- Summary: Soft-delete quiz.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizDeleteManaged): `admin`, `func`, `assistant`.
- Path params:
  - `quizId: uuid`
- Response `data`: none (success envelope only).

## POST `/quizzes/join/{joinCode}`
- Summary: Resolve join code and return join preview.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizJoinByCode): `admin`, `func`, `assistant`, `student`.
- Path params:
  - `joinCode: string`
- Body: none.
- Response `data`: `JoinQuizPreviewView`
  - `{ quizId, title, kind, startsAt }`

## GET `/quizzes/join/{joinCode}/attempts/me/result`
- Summary: Get current user result for quiz by join code.
- Auth: Bearer token required.
- Roles (AuthzAction::QuizViewAttemptResultByCode): `admin`, `func`, `assistant`, `student`.
- Path params:
  - `joinCode: string`
- Policy: access depends on course membership and student ownership checks.
- Response `data`: `AttemptResultView`

## Output Shape: `QuizView`
- `{ id, title, kind, joinCode, questionCount, certaintyTable, attemptDurationMinutes, startsAt, closedAt, createdAt, course }`
- `course` is embedded course entity data.

## Common Domain Errors
- `404`: quiz not found.
- `403`: forbidden quiz access.
- `400`: invalid create payload or mode mismatch.
- `409`: quiz closed or locked by existing attempts.
