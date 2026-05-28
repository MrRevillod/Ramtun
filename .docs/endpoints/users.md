# Users Endpoints

## GET `/users/me`
- Summary: Get current authenticated user.
- Auth: Bearer token required.
- Roles: any authenticated role.
- Response `data`:
  - `{ id, username, name, email, role }`

## GET `/users`
- Summary: List users with optional search/role filter.
- Auth: Bearer token required.
- Roles (AuthzAction::UserListAdmin): `admin`, `func`.
- Query params:
  - `search?: string`
  - `roles?: string` (comma-separated, e.g. `student,func`)
- Response `data`: `User[]`.

## GET `/users/collaborator-candidates`
- Summary: List users eligible as quiz collaborators.
- Auth: Bearer token required.
- Roles (AuthzAction::UserListCollaboratorCandidates): `admin`, `func`, `student`.
- Query params:
  - `search?: string`
- Response `data`: `User[]`.

## PATCH `/users/{userId}/role`
- Summary: Update user role to a manageable role.
- Auth: Bearer token required.
- Roles (AuthzAction::UserManageRole): `admin`, `func`.
- Path params:
  - `userId: uuid`
- Body:

```json
{
  "role": "func"
}
```

- Allowed values in request: `student | func`.
- Response `data`: updated `User`.
