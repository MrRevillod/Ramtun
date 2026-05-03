# Auth Endpoints

## POST `/auth/login`
- Summary: Authenticate with username/password.
- Auth: Public.
- Body:

```json
{
  "username": "user",
  "password": "secret"
}
```

- Response `data`:
  - `{ user, accessToken, refreshToken }`
  - `user`: `{ id, username, name, email, role }`

## POST `/auth/refresh`
- Summary: Refresh access and refresh tokens.
- Auth: Public endpoint, but requires bearer refresh token in header.
- Header:
  - `Authorization: Bearer <refreshToken>`
- Body: none.
- Response `data`:
  - `{ accessToken, refreshToken }`

## POST `/auth/logout`
- Summary: Revoke current session.
- Auth: Bearer token required.
- Body: none.
- Response `data`: none.

## Common Errors
- `401`: missing/invalid token.
- `403`: forbidden session operation.
