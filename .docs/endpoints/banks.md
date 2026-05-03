# Banks Endpoints

## GET `/banks/course/{courseId}`
- Summary: List question banks in a course.
- Auth: Bearer token required.
- Roles (AuthzAction::BankList): `admin`, `func`, `assistant`, `student`.
- Policy: user must have access to the course.
- Path params:
  - `courseId: uuid`
- Response `data`: `QuestionBank[]`.

## GET `/banks/{bankId}`
- Summary: Get one question bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankRead): `admin`, `func`, `assistant`, `student`.
- Policy: user must have access to the bank/course.
- Path params:
  - `bankId: uuid`
- Response `data`: `QuestionBank`.

## POST `/banks`
- Summary: Create question bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankCreate): `admin`, `func`, `assistant`, `student`.
- Policy: service enforces course accessibility.
- Body:

```json
{
  "courseId": "uuid",
  "name": "Banco Algebra",
  "questions": [
    {
      "prompt": "2 + 2 = ?",
      "options": ["3", "4"],
      "answerIndex": 1,
      "images": []
    }
  ]
}
```

- Response `data`: none.

## PATCH `/banks/{bankId}`
- Summary: Update bank name and/or questions.
- Auth: Bearer token required.
- Roles (AuthzAction::BankUpdate): `admin`, `func`, `assistant`, `student`.
- Path params:
  - `bankId: uuid`
- Body (at least one field):

```json
{
  "name": "Banco actualizado",
  "questions": []
}
```

- Response `data`: none.

## DELETE `/banks/{bankId}`
- Summary: Soft-delete bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankDelete): `admin`, `func`, `assistant`, `student`.
- Path params:
  - `bankId: uuid`
- Response `data`: none.

## Common Domain Errors
- `409`: locked by running quiz or invalid question count after bank update sync.
- `404`: bank not found.
- `400`: invalid question payload.
