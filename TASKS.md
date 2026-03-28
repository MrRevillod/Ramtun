# TASKS

## Objetivo

Refactorizar el backend para que el dominio, la API REST y el sistema de autorizacion sean consistentes, mantenibles y profesionales.

Orden recomendado:

1. Refactor del dominio y del modelo de datos.
2. Refactor de la API REST y de los contratos HTTP.
3. Refactor de casos de uso y views.
4. Refactor del sistema de autorizacion/RBAC.
5. Tests y limpieza.

Importante: no empezar por RBAC. Primero hay que arreglar el modelo del dominio, porque hoy `authorized_user_ids` mezcla colaboradores del quiz con usuarios autorizados para rendir.

---

## Fase 1 - Refactor del modelo de datos

### 1.1. Rediseñar el esquema relacional

- [x] Dejar de usar `authorized_user_ids` como fuente principal de permisos en `apps/server/src/quizzes/entity.rs`.
- [x] Agregar `owner_id UUID NOT NULL REFERENCES users(id)` a la tabla `quizzes`.
- [x] Agregar `updated_at TIMESTAMPTZ NOT NULL` a la tabla `quizzes`.
- [x] Evaluar si `join_code` debe seguir siendo unico global o unico por quiz activo; documentar la decision.

### 1.2. Crear tablas nuevas

- [x] Crear migracion para `quiz_collaborators` con columnas:
  - `quiz_id`
  - `user_id`
  - `created_at`
  - `UNIQUE (quiz_id, user_id)`
- [x] Crear migracion para `quiz_attempts` con columnas:
  - `id`
  - `quiz_id`
  - `student_id`
  - `started_at`
  - `submitted_at NULL`
  - `status`
  - `UNIQUE` segun regla elegida para cantidad de intentos por estudiante
- [x] Crear migracion para `quiz_answers` con columnas:
  - `attempt_id`
  - `question_id`
  - `answer_index`
  - `certainty_level NULL`
  - `UNIQUE (attempt_id, question_id)`

### 1.3. Agregar restricciones e indices

- [x] Agregar indice por `quizzes.owner_id`.
- [x] Agregar indice por `quiz_collaborators.user_id`.
- [x] Agregar indice por `quiz_attempts.student_id`.
- [x] Agregar indice por `quiz_attempts.quiz_id`.
- [x] Agregar `CHECK` o enum para el estado de intento (`in_progress`, `submitted`).

### 1.4. Plan de migracion de datos

- [ ] Crear estrategia para migrar datos desde `authorized_user_ids`.
- [ ] Definir si los usuarios de `authorized_user_ids` actuales se transforman en `collaborators` o si se necesita una migracion manual.
- [ ] Si no se puede inferir con seguridad, crear una migracion que preserve los datos viejos temporalmente y dejar la conversion como paso manual documentado.

### 1.5. Refactor de entidades Rust

- [x] Reemplazar el modelo actual `Quiz` en `apps/server/src/quizzes/entity.rs` por entidades separadas:
  - `QuizEntity`
  - `QuizCollaboratorEntity`
  - `QuizAttemptEntity`
  - `QuizAnswerEntity`
- [x] Rediseñar la estructura de preguntas para que cada pregunta tenga un identificador estable y no dependa de su indice dentro del arreglo.
- [x] Evitar que las entidades de persistencia se usen directamente como respuesta HTTP.

Criterio de cierre:

- Las migraciones nuevas existen, compilan y representan claramente owner, collaborators e attempts.

---

## Fase 2 - Rediseñar la API REST

### 2.1. Definir recursos principales

- [x] Agrupar endpoints por recurso, no por accion suelta.
- [x] Definir rutas base para:
  - `/auth`
  - `/users`
  - `/quizzes`
  - `/attempts`

### 2.2. Reemplazar endpoints de quizzes

- [ ] Mantener `POST /auth/login`, `POST /auth/refresh`, `POST /auth/logout`.
- [x] Agregar `GET /users/me` para devolver el usuario autenticado.
- [x] Reemplazar `GET /quizzes/mine` por una ruta mas clara, por ejemplo `GET /quizzes/me`.
- [x] Reemplazar `GET /quizzes/code/{code}` por `POST /quizzes/join-by-code` o `POST /quizzes/access-by-code`.
- [x] Mantener `POST /quizzes` para crear quiz.
- [x] Agregar `GET /quizzes/{quizId}` para detalle del quiz.
- [x] Agregar `PATCH /quizzes/{quizId}` para editar metadatos del quiz.
- [x] Definir en el contrato HTTP que las respuestas de intentos usan `questionId` y no la posicion de la pregunta.

### 2.3. Agregar subrecursos de colaboracion

- [x] Agregar `GET /quizzes/{quizId}/collaborators`.
- [x] Agregar `PUT /quizzes/{quizId}/collaborators/{userId}`.
- [x] Agregar `DELETE /quizzes/{quizId}/collaborators/{userId}`.

### 2.4. Agregar recursos de intentos

- [x] Agregar `POST /quizzes/{quizId}/attempts` para iniciar intento.
- [x] Agregar `GET /quizzes/{quizId}/attempts/me` para ver intento propio del quiz.
- [x] Agregar `POST /attempts/{attemptId}/submit` para entregar el quiz.

### 2.5. Ajustar gestion de usuarios

- [x] Reemplazar `PATCH /users/role` por `PATCH /users/{userId}/role`.
- [x] Cambiar el DTO para que la ruta use `userId` y no `username` como identificador principal.
- [x] Revisar si `GET /users` y `GET /users/quiz-candidates` deben permanecer separados o si uno puede convertirse en una query mas expresiva.

### 2.6. Estandarizar errores HTTP

- [x] Definir errores consistentes para `400`, `401`, `403`, `404`, `409`, `422`.
- [x] Usar `409 Conflict` para colisiones como join code duplicado o estado invalido.
- [x] Usar `403 Forbidden` para falta de permisos.
- [ ] Usar `404 Not Found` cuando el recurso no existe o no es visible segun la politica elegida.

Criterio de cierre:

- Existe un contrato REST claro, y los endpoints expresan recursos y subrecursos reales del dominio.

---

## Fase 3 - Separar entidades, DTOs y views

### 3.1. Reorganizar structs del dominio

- [x] Separar en cada modulo los siguientes tipos:
  - entidades de persistencia
  - DTOs de entrada (commands)
  - views de salida (queries/read models)
- [ ] Evitar `pub use *` indiscriminado en modulos donde mezcles entidades, DTOs y errores.

### 3.2. Crear views especificas para quizzes

- [x] Crear una `QuizSummaryView` para listados.
- [x] Crear una `QuizDetailView` para owner/collaborator.
- [x] Crear una `QuizParticipantView` para `student` cuando se une a rendir.
- [x] Asegurar que `QuizParticipantView` incluya `questionId` por pregunta para soportar orden aleatorio.
- [x] Asegurar que la view del estudiante nunca incluya respuestas correctas.
- [x] Asegurar que la view de gestion si incluya toda la informacion necesaria para editar el quiz.

### 3.3. Crear DTOs especificos por caso de uso

- [x] Crear `CreateQuizRequest`.
- [x] Crear `UpdateQuizRequest`.
- [x] Crear `AddCollaboratorRequest` si el `userId` no va solo en path.
- [x] Crear `StartAttemptRequest` si hace falta metadata adicional.
- [x] Crear `SubmitAttemptRequest` con la lista de respuestas.
- [x] Crear `UpdateUserRoleRequest`.

### 3.4. Limpiar DTOs actuales

- [x] Refactorizar `apps/server/src/quizzes/dtos.rs` para separar create, update, join y submit.
- [x] Refactorizar DTOs de preguntas e intentos para usar `questionId` en vez de `questionIndex`.
- [x] Eliminar nombres demasiado genericos como `CreateQuizDto` si ya no representan el contrato final.
- [x] Refactorizar `apps/server/src/users/dtos.rs` para usar ids estables y no depender de `username` para operaciones administrativas.

Criterio de cierre:

- Ninguna entidad de base de datos se usa directamente como contrato HTTP de salida.

---

## Fase 4 - Refactor de casos de uso y modulos de negocio

### 4.1. Refactor del modulo `quizzes`

- [x] Dividir responsabilidades de `QuizService` en casos de uso mas claros si empieza a crecer demasiado.
- [x] Crear funciones separadas para:
  - crear quiz
  - obtener quiz para gestion
  - listar quizzes gestionables por usuario
  - actualizar quiz
  - agregar colaborador
  - remover colaborador
- [x] Modificar `QuizRepository` para que deje de consultar por arrays de UUID y pase a consultar tablas relacionales.

### 4.2. Crear modulo nuevo `attempts`

- [x] Crear `apps/server/src/attempts/mod.rs`.
- [x] Crear `controller.rs`, `service.rs`, `repository.rs`, `entity.rs`, `dtos.rs`, `errors.rs`.
- [x] Registrar el modulo en `apps/server/src/main.rs`.
- [x] Registrar el adapter/controller y los componentes en el modulo.

### 4.3. Implementar flujo de intento

- [x] Implementar iniciar intento.
- [x] Implementar logica para entregar preguntas en orden aleatorio al estudiante.
- [x] Implementar guardar respuestas al enviar.
- [x] Validar respuestas usando `questionId` y no el orden de presentacion.
- [x] Bloquear re-envio si el intento ya fue enviado.
- [x] Validar vencimiento por `attempt_duration_minutes`.

### 4.4. Refactor de users

- [x] Mantener en `users` solo responsabilidades de lectura/gestion de usuarios.
- [x] Sacar de `users` cualquier logica de autorizacion de quiz que no le corresponda.
- [x] Actualizar el flujo de cambio de rol para que solo toque `student <-> assistant`.

Criterio de cierre:

- Los modulos del backend reflejan casos de uso reales y tienen responsabilidades claras.

---

## Fase 5 - Refactor profesional de autorizacion/RBAC

### 5.1. Reemplazar el modelo actual de autorizacion

- [x] Dejar de depender exclusivamente de `AuthzAction` global en `apps/server/src/authz/mod.rs`.
- [ ] Crear un modulo nuevo `authorization` o refactorizar `authz` hacia una estructura mas rica.
- [x] Dejar disponible el `User` autenticado actual en la request para autorizacion por recurso.

### 5.2. Crear permisos globales explicitos

- [x] Definir un enum o conjunto de permisos globales, por ejemplo:
  - `QuizCreate`
  - `UserAssignAssistantRole`
  - `QuizAttemptCreate`
  - `QuizAttemptSubmitOwn`
- [x] Definir un servicio de permisos globales por rol.
- [x] Dejar documentado que el rol solo da el permiso base, pero no decide por si mismo el acceso a un recurso concreto.

### 5.3. Crear policies por recurso

- [x] Crear `QuizPolicy` con metodos como:
  - `can_read_managed_quiz`
  - `can_update_quiz`
  - `can_manage_collaborators`
  - `can_join_quiz`
- [x] Crear `AttemptPolicy` con metodos como:
  - `can_start_attempt`
  - `can_submit_attempt`
- [x] Crear `UserPolicy` con metodos como:
  - `can_assign_assistant_role`

### 5.4. Redefinir guards e interceptors

- [x] Mantener `SessionCheck` como guard de autenticacion.
- [x] Crear un guard liviano para dejar el `User` autenticado disponible en la request.
- [x] Crear un guard de permiso global para operaciones que no dependen de recurso concreto.
- [x] No hacer en el guard la logica fina de quiz/attempt; mover eso a policies y servicios de aplicacion.

### 5.5. Cambiar el uso de autorizacion en controllers

- [x] En cada endpoint, cargar el recurso primero cuando sea necesario.
- [x] Llamar a la policy correcta antes de ejecutar el caso de uso.
- [x] No repetir logica de permisos manualmente en cada controller; encapsularla en policies.

### 5.6. Eliminar permisos viejos o ambiguos

- [x] Eliminar acciones legacy como `ListMyQuizzes`, `FinalizeQuiz`, `ViewQuizResults` si ya no representan el modelo nuevo.
- [x] Renombrar cualquier permiso que dependa del endpoint viejo y no del recurso real.

Criterio de cierre:

- El sistema de autorizacion combina permisos globales por rol y policies por recurso.

---

## Fase 6 - Refactor de controllers y repositories concretos

### 6.1. Controllers

- [x] Reescribir `apps/server/src/quizzes/controller.rs` para adaptarlo a la nueva API REST.
- [x] Crear `apps/server/src/attempts/controller.rs`.
- [x] Reescribir `apps/server/src/users/controller.rs` para usar `userId` en path al cambiar roles.
- [x] Revisar `apps/server/src/auth/controller.rs` y agregar `GET /me` si decides ubicarlo ahi o en otro modulo.

### 6.2. Repositories

- [x] Reescribir `QuizRepository` para usar joins y tablas auxiliares en vez de `authorized_user_ids`.
- [ ] Crear `QuizCollaboratorRepository` si el repositorio unico se vuelve demasiado grande.
- [x] Crear `AttemptRepository`.
- [ ] Agregar consultas optimizadas para listados del dashboard.

### 6.3. Errores de dominio

- [x] Revisar `apps/server/src/quizzes/errors.rs` y eliminar errores que ya no representen bien el dominio.
- [x] Crear `AttemptError`.
- [x] Crear errores de autorizacion mas expresivos cuando corresponda.
- [x] Revisar `apps/server/src/shared/errors.rs` para asegurar que mapea correctamente errores de dominio a respuestas HTTP.

Criterio de cierre:

- Controllers, repositories y errores estan alineados con el nuevo dominio y no con la implementacion antigua.

---

## Fase 7 - Actualizar frontend minimo para no romper flujos

### 7.1. Ajustar clientes HTTP

- [x] Actualizar `apps/client/src/lib/features/quiz/quiz.service.ts` a las rutas nuevas.
- [x] Actualizar `apps/client/src/lib/features/users/users.service.ts` a las rutas nuevas.
- [x] Agregar un `attempt.service.ts` para iniciar y enviar intentos.

### 7.2. Ajustar flujos UI

- [x] Cambiar el flujo de join para que obtenga un quiz participant view sin respuestas correctas.
- [x] Ajustar el flujo del estudiante para enviar respuestas con `questionId`.
- [x] Cambiar `JoinedQuizRunner.svelte` para que deje de guardar solo en memoria y envie el intento real al backend.
- [x] Ajustar `CreateQuizForm.svelte` para reflejar que el quiz solo gestiona colaboradores y no asignaciones de estudiantes.
- [x] Ajustar `CreatedQuizList.svelte` para listar quizzes gestionables segun owner/collaborator.
- [x] Ajustar `AssistantManager.svelte` si cambia el endpoint de gestion de roles.

### 7.3. Ajustar tipos TypeScript

- [x] Crear tipos nuevos para `QuizSummary`, `QuizDetail`, `QuizParticipant`, `Attempt`, `Grade`.
- [x] Eliminar tipos frontend que dependan del contrato viejo del backend.

Criterio de cierre:

- El frontend vuelve a funcionar con la nueva API sin depender de endpoints legacy.

---

## Fase 8 - Tests obligatorios

### 8.1. Tests de dominio y policies

- [x] Crear tests unitarios para `QuizPolicy`.
- [x] Crear tests unitarios para `AttemptPolicy`.
- [x] Crear tests unitarios para `UserPolicy`.
- [x] Cubrir al menos estos casos:
  - student puede unirse si corresponde
  - student no puede editar quiz
  - assistant puede editar quiz colaborativo
  - assistant no puede editar quiz ajeno
  - func puede promover assistant
  - assistant no puede promover assistant

### 8.2. Tests de integracion HTTP

- [ ] Testear `POST /quizzes` con `assistant` y `func`.
- [ ] Testear que `student` reciba `403` en `POST /quizzes`.
- [ ] Testear `PATCH /quizzes/{quizId}` con collaborator y no-collaborator.
- [ ] Testear `POST /quizzes/{quizId}/attempts`.
- [ ] Testear `POST /attempts/{attemptId}/submit`.
- [ ] Testear `PATCH /users/{userId}/role`.

### 8.3. Tests de repositorio

- [ ] Testear queries de listados y joins importantes.
- [ ] Testear constraints unicas y estados invalidos.

Criterio de cierre:

- Las reglas de permisos y los endpoints criticos estan cubiertos por tests automatizados.

---

## Fase 9 - Limpieza final

### 9.1. Remover codigo legacy

- [ ] Eliminar rutas viejas que ya no se usen.
- [x] Eliminar enums de permisos viejos que ya no tengan sentido.
- [x] Eliminar campos y consultas que dependan de `authorized_user_ids`.
- [ ] Eliminar adapters, DTOs o services obsoletos.

### 9.2. Documentacion final

- [ ] Actualizar README con arquitectura nueva.
- [ ] Actualizar la documentacion existente si el diseno final difiere de `.docs`.

### 9.3. Verificacion final

- [x] Ejecutar `cargo check`.
- [x] Ejecutar tests del backend.
- [ ] Ejecutar `npm`/`vite` checks del frontend.
- [ ] Validar manualmente los 3 flujos principales:
  - profesor crea y edita quiz
  - ayudante gestiona quiz colaborativo
  - estudiante se une y entrega un intento

---

## Orden practico de ejecucion

1. Fase 1
2. Fase 2
3. Fase 3
4. Fase 4
5. Fase 5
6. Fase 6
7. Fase 7
8. Fase 8
9. Fase 9

---

## Riesgos a vigilar durante el refactor

- `authorized_user_ids` hoy tiene semantica ambigua; no asumir una migracion automatica sin revisar datos reales.
- Si cambias rutas del backend, el frontend se va a romper hasta que actualices servicios y tipos.
- Si implementas policies sin separar views, puedes terminar filtrando datos sensibles demasiado tarde.
- Si mantienes entidades DB como responses HTTP, el refactor quedara a medio camino.
- Si dejas demasiada logica de autorizacion dentro de controllers, el sistema seguira dificil de mantener.

---

## Fase futura

Cuando el refactor de dominio, API y RBAC este estable, se podra abrir una fase futura para:

- correccion automatica
- calculo de puntajes
- vistas de resultados
