# Courses Endpoints

## GET `/courses`
- Summary: List courses available to current user.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseList): `admin`, `func`, `assistant`.
- Response `data`: `CourseView[]`.

## GET `/courses/{courseId}`
- Summary: Get one course with members.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseRead): `admin`, `func`, `assistant`.
- Path params:
  - `courseId: uuid`
- Response `data`: `CourseView`.

## POST `/courses`
- Summary: Create a new course.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseCreate): `admin`, `func`, `assistant`.
- Body:

```json
{
  "name": "Matematica Discreta",
  "code": "MAT123",
  "year": 2026
}
```

- Response `data`: `CourseView`.

## DELETE `/courses/{courseId}`
- Summary: Soft-delete a course.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseDelete): `admin`, `func`.
- Path params:
  - `courseId: uuid`
- Response `data`: none.

## GET `/courses/{courseId}/members`
- Summary: List members of a course.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseRead): `admin`, `func`, `assistant`.
- Path params:
  - `courseId: uuid`
- Response `data`: `CourseMemberView[]`.

## POST `/courses/{courseId}/members`
- Summary: Add member to course.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseManageMembers): `admin`, `func`.
- Path params:
  - `courseId: uuid`
- Body:

```json
{
  "userId": "uuid"
}
```

- Response `data`: none.

## DELETE `/courses/{courseId}/members/{userId}`
- Summary: Remove member from course.
- Auth: Bearer token required.
- Roles (AuthzAction::CourseManageMembers): `admin`, `func`.
- Path params:
  - `courseId: uuid`
  - `userId: uuid`
- Response `data`: none.

## Output Shapes
- `CourseView`: `{ id, name, code, year, members }`
- `CourseMemberView`: `{ userId, username, name, role }`
