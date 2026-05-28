# Frontend Rebuild Plan (Ramtun)

## 1) Objetivo

Reconstruir el frontend completo con estandares profesionales, manteniendo:

- diseno sobrio en escala de grises (como login actual)
- arquitectura limpia por responsabilidades
- contratos API alineados a `.docs/endpoints/*.md`
- `formisch` para formularios
- `valibot` para schemas
- `tanstack query` para queries/mutations
- funcionalidad v1: reanudar intento en la pregunta donde quedo

---

## 2) Principios de arquitectura

- `shared/http`: transporte, interceptors, retry/refresh, normalizacion de errores
- `shared/auth/session.manager`: lectura/escritura de sesion en un unico punto
- `shared/errors`: `AppError` + `getErrorMessage(error: unknown)`
- `features/*/*.service.ts`: llamadas HTTP por dominio (`Result` + `OrThrow`)
- `routes/components`: TanStack Query (`onSuccess/onError`), navegacion, UX feedback
- No mezclar UI side-effects dentro de `shared/http` ni en services de dominio
- Backend como fuente de verdad para authz/policy (`403` siempre manejado)

---

## 3) Diseno del sistema visual

## 3.1 Direccion visual

- Base: `white`, `zinc-50`, `zinc-100`, `zinc-900`
- Primario: negro (`bg-black`, `text-white`) para acciones principales
- Bordes: `zinc-200/300`
- Semanticos:
  - exito: `emerald`
  - error: `red`
  - informativo: `blue`
- Tipografia: Palatino (consistente con login actual)

## 3.2 Componentes base compartidos (UI Kit interno)

Crear en `src/lib/shared/ui/`:

- `PageHeader.svelte`
- `SectionCard.svelte`
- `DataTable.svelte`
- `FormField.svelte`
- `EmptyState.svelte`
- `ErrorState.svelte`
- `LoadingState.svelte`
- `StatusBadge.svelte`
- `ConfirmDialog.svelte`

Regla: todas las vistas nuevas deben usar estos bloques antes de crear variantes ad-hoc.

---

## 4) Mapa de rutas y roles

## 4.1 Rutas

- `/(protected)/join`
- `/(protected)/results`
- `/(protected)/courses`
- `/(protected)/courses/[courseId]/members`
- `/(protected)/courses/[courseId]/banks`
- `/(protected)/courses/[courseId]/quizzes`
- `/(protected)/courses/[courseId]/quizzes/[quizId]/attempts`
- `/(protected)/admin/users`

## 4.2 Roles esperados

- `student`: join + attempt + results + courses if member
- `func`: student + courses/members/banks/quizzes/attempts + admin/users (gestion roles globales)
- `admin`: full access incl. admin dashboard con todos los cursos y usuarios

Notas:
- `assistant` ya no es un rol global; es un rol de miembro de curso (`CourseMemberRole::Assistant`).
- Un student puede ser miembro de curso con rol `assistant` o `func`.
- `CourseManageMembers`: `admin`, `func` (y policy de manager member en backend)

---

## 5) Estructura por modulos

## 5.1 Auth

- Login/logout/bootstrap estable
- TanStack mutation para login/logout
- Guards en layouts/loaders
- Interceptor refresh ya desacoplado (mantener)

## 5.2 Courses + Members

- List/create course
- Course detail (contexto de navegacion)
- Members list/add/remove
- Visibility de acciones por rol/policy

## 5.3 Banks

- List por curso
- Create/update/delete bank
- Forms con `formisch`
- Schemas con `valibot`

## 5.4 Quizzes (gestion)

- List managed quizzes por curso/contexto
- Create quiz (course + bankIds + kind + config)
- Delete quiz

## 5.5 Join + Attempt

- Join por codigo (`/quizzes/join/{joinCode}`)
- Initialize attempt
- Save answer por pregunta
- Submit attempt

## 5.6 Results

- View result por `attemptId`
- View result por `joinCode`
- Cards de resumen + detalle por pregunta

## 5.7 Admin Users

- List/search users
- Promote/demote roles globales (`func/admin`)

---

## 6) Reglas tecnicas obligatorias

- Formularios:
  - `formisch` para estado/submit
  - `valibot` para validacion de payload
- Datos remotos:
  - `tanstack query` en componentes o archivos `*.queries.ts` cuando haya reuse
- Services:
  - metodos `foo()` -> `AppResult<T>`
  - metodos `fooOrThrow()` -> `Promise<T>`
- Errores:
  - UI usa `getErrorMessage(error)`
  - nada de casts manuales tipo `as Parameters<...>`

---

## 7) Funcionalidad v1 a conservar: reanudar intento

Implementacion:

- Persistir `currentQuestionIndex` por `attemptId`
- Key sugerida: `attempt-progress:${attemptId}`
- Guardar al cambiar de pregunta
- Restaurar al montar runner:
  - si indice valido: usarlo
  - fallback: primera no respondida
- Limpiar al submit exitoso

---

## 8) Fases de ejecucion

## Fase 1: Foundation

- Estructura de rutas + layout + componentes base UI
- Guards por rol/contexto
- Convencion de queries/mutations

## Fase 2: Courses/Members

- Implementar paginas funcionales completas
- Validar roles y policy (course manager member)

## Fase 3: Banks

- CRUD de bancos por curso

## Fase 4: Quizzes (gestion)

- Crear/listar/eliminar quizzes

## Fase 5: Join + Attempt + Submit

- Runner completo + certeza + persistencia de respuestas

## Fase 6: Results

- Resultados por attempt/joinCode

## Fase 7: Admin Users

- Gestion global de roles

## Fase 8: Hardening

- UX consistency pass
- handling de errores edge-case
- chequeos finales (`pnpm check`, lint, smoke manual por rol)

---

## 9) Definicion de terminado (DoD)

- `pnpm check` sin errores
- rutas y vistas alineadas a `.docs/endpoints/*.md`
- manejo de errores uniforme con `AppError/getErrorMessage`
- no hay logica de presentacion dentro de services/http layer
- flujo auth + refresh estable
- resume attempt v1 funcionando
- QA manual minimo por rol:
  - student
  - func
