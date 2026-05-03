# API Conventions

## Base URL
- All endpoints are served behind `/api` from nginx.
- Example full URL: `/api/quizzes/me`.

## Response Envelope
- Every endpoint responds with the same envelope:

```json
{
  "code": 200,
  "success": true,
  "message": "OK",
  "timestamp": "2026-05-03T00:00:00Z",
  "data": {}
}
```

- Frontend should read payloads from `data`.

## Auth
- Protected endpoints require `Authorization: Bearer <accessToken>`.
- Public endpoints: `POST /auth/login`, `POST /auth/refresh`.

## Roles
- Roles in backend: `student`, `func`, `assistant`, `admin`.
- `AuthzGuard` validates role access by action.
- Service policies can further restrict access based on resource ownership/membership.

## Serialization
- JSON fields are camelCase.
- `quiz.kind` is lowercase enum: `traditional | certainty`.

## Error Handling
- Common statuses used by modules: `400`, `401`, `403`, `404`, `409`.
- Validation and domain errors are returned via the standard envelope with `success: false`.
