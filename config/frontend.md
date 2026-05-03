
# Instrucciones de frontend page

## Resumen

Ramtun es una plataforma de tests de certeza academica. En este documento te definiré los requerimientos de la web.

La plataforma cuenta con cursos, quizzes, question banks y otros, sin embargo, los cursos no almacenan estudiantes, si no que es curso está pensado como un contenedor de quizzes y bancos de preguntas, para que el  profesor y ayudantes puedan organizar su material. Un estudiante no se une a un curso, sino que se une a un quiz, y el quiz pertenece a un curso.

## Usuarios

La plataforma tiene 4 tipos de usuarios, que se  traducen a roles, "student", "func", "assistant" y "admin".

### Student

Un student debe poder:
- Iniciar sesión
- Unirse a un quiz (con un código de quiz)
- Enviar un quiz (finalizar intento) 
- Ver resultados de un quiz (con un código de quiz)


### Assistant

Un assistant, es un student que ha sido promovido a "ayudante" en un curso X. Un assistant debe poder:

- Hacer todo lo que un student puede hacer
- Crear quizzes para el curso X
- Finalizar quizzes para el curso X
- Ver resultados de los quizzes del curso X
- Administrar members del curso, añadir o remover.
- Crear un curso.
- Crear bancos de preguntas para el curso X
- Eliminar bancos de preguntas para el curso X
- Eliminar quizzes para el curso X

### Func
Un func es un profesor. Un func debe poder:
- Hacer todo lo que un assistant puede hacer
- Promover a un student a assistant (globalmente)


### Admin
Un admin es un super usuario, que tiene acceso a todo. Un admin debe poder:
- Hacer todo lo que un func puede hacer

## Diseño e interfaz de la página web

Actualmente hay componentes en la página web, sin embargo, cumplia las necesidades básicas del MVP inicial. La siguiente meta es mejorar el diseño y la interfaz de la web. Por ejemplo, haciendo que se parezca más a una plataforma de educación, tipo Educa Blackboard.

Mantendriamos la paleta de colores sobria en escala de grises, pero mejorariamos en general la experiencia de usuarios, haciendo que sea más fácil navegar por la página, y que sea más agradable a la vista.

La web utiliza tailwind y fuente Palatino.

## Analisis del frontend actual (para rediseno desde cero)

### Estado general

- El frontend actual tiene una base tecnica util (auth, cliente HTTP, shell de layout), pero los modulos de dominio de quiz/attempt quedaron desalineados del backend actual.
- El rediseno se hara desde cero en dominio funcional, manteniendo solo base comun: auth, axios/http y estructura general.

### Hallazgos de UX actuales que si conviene conservar como lenguaje visual

#### 1) Runner de preguntas (intento)

Referencia actual: `apps/client/src/lib/features/quiz/components/JoinedQuizRunner.svelte`

- Estructura acertada:
  - encabezado con titulo, progreso y contador de respuestas
  - temporizador visible y accion de salida
  - bloque principal con pregunta, imagenes y opciones
  - pie con estado de guardado y accion de avanzar/finalizar
- Patron de seleccion util:
  - opcion seleccionada en negro (`bg-black text-white`)
  - opcion no seleccionada en blanco con borde gris y hover suave
- Feedback en tiempo real:
  - mensajes tipo "Guardando respuesta" / "Respuesta guardada"
  - validacion de avance segun estado de respuesta

#### 2) Bloque de certeza

Referencia actual: `JoinedQuizRunner.svelte` y `JoinQuizInstructions.svelte`

- Niveles: `Baja`, `Media`, `Alta`.
- Comportamiento que debe mantenerse:
  - en quiz de certeza, no se permite continuar/finalizar sin certeza valida cuando corresponde.
- Lenguaje visual recomendable:
  - 3 botones en grilla con misma jerarquia visual que opciones.

#### 3) Visor de resultados

Referencia actual: `apps/client/src/lib/features/quiz/components/AttemptResultView.svelte`

- Resumen superior correcto:
  - nota final
  - puntaje total
  - fecha/hora de envio
- Detalle por pregunta util:
  - estado `Correcta` (verde) / `Incorrecta` (rojo)
  - resaltado por opcion:
    - correcta: verde
    - seleccion incorrecta del estudiante: rojo
  - indicadores auxiliares:
    - puntos otorgados por respuesta
    - marca de "Tu seleccion"
  - mensaje explicito cuando no hubo respuesta.

#### 4) Sistema visual global

Referencia actual: `apps/client/src/routes/layout.css`

- Mantener direccion visual sobria en escala de grises.
- Mantener tipografia Palatino.
- Mantener semantica cromatica en feedback:
  - verde = correcto
  - rojo = incorrecto
  - azul = etiqueta informativa secundaria

### Restriccion funcional explicita

- Las preguntas y opciones NO deben soportar HTML enriquecido ni renderizado de markup.
- Todo contenido de pregunta/opciones se tratara como texto plano.
- En el rediseno no debe incluirse soporte legacy de rich text.

### Criterio para la reconstruccion

- Rehacer desde cero los modulos de dominio frontend:
  - quizzes
  - attempts
  - courses
  - banks
- Mantener solo base compartida existente:
  - auth
  - cliente HTTP
  - shell global
- Contratos API obligatorios: usar `.docs/endpoints/*.md` como fuente unica.

## Estandares de arquitectura frontend

### Principios

- Separacion de responsabilidades estricta por capa.
- Los modulos de UI no deben conocer detalles de transporte HTTP.
- Los services de dominio no deben mezclar logica de red con logica de presentacion.
- Todo error debe circular con tipos explicitos y consistentes.

### Capas y responsabilidades

- `shared/http/*`
  - Define `apiClient`, contrato de request y normalizacion de errores.
  - Mantiene interceptors y politicas de retry/refresh.
  - No maneja toasts, no navega, no renderiza UI.
- `shared/auth/session.manager.ts`
  - Punto unico para leer/escribir estado de sesion.
  - Operaciones: obtener tokens/usuario, setear sesion, actualizar tokens, limpiar sesion.
- `shared/http/refresh.coordinator.ts`
  - Coordina refresh concurrente (single-flight) para evitar condiciones de carrera.
- `features/*/*.service.ts`
  - Encapsulan llamadas de red por feature.
  - Exponen metodos `Result` y metodos `OrThrow` para integracion limpia con TanStack Query.
  - No deben mostrar toasts ni decidir navegacion.
- `routes/*` y componentes
  - Definen `createQuery/createMutation`.
  - Manejan side effects de UI (`onSuccess`, `onError`, redirects, feedback visual).

### Manejo de errores (profesional)

- Usar `neverthrow` como estandar unico de errores de dominio/API.
- Modelo de error unico: `AppError` discriminado por `kind` (`auth`, `http`, `network`, `decode`, `unknown`).
- La capa HTTP debe mapear errores tecnicos a `AppError` consistente.
- Para UI, usar `getErrorMessage(error: unknown)` como adaptador de mensajes de usuario.
- Evitar `try/catch` en handlers de formulario cuando TanStack Query puede gestionar `onError`.

### Integracion con TanStack Query

- Las mutations/queries deben recibir funciones limpias desde los services.
- Preferir metodos `xxxOrThrow()` en `mutationFn/queryFn` para evitar unwrap manual en componentes.
- Side effects en `onSuccess/onError`:
  - login exitoso: setSession + redirect.
  - logout (ok/error): limpiar sesion local y redirigir a login.
- Invalidaciones y cache se implementan en capa de componentes/queries, no en services.

### Convenciones de implementacion

- Mantener sintaxis de metodos TypeScript en services (`public async foo()`), no propiedades flecha por defecto.
- Mantener naming consistente:
  - `foo()`: retorna `AppResult<T>`.
  - `fooOrThrow()`: retorna `T` y lanza `AppError`.
- Mantener todo payload en camelCase y alineado a `.docs/endpoints/*.md`.
- No introducir soporte de rich text/HTML en preguntas u opciones.

### Criterios de calidad minima

- `pnpm check` debe pasar en cada cambio.
- El flujo auth debe mantenerse estable respecto al comportamiento esperado inicial:
  - login
  - refresh con retry automatico en `401`
  - logout
  - redireccion de rutas protegidas sin sesion
- Cambios de arquitectura no deben alterar contratos de backend ni romper UX base.

## Plan de views por roles (alineado a authz real)

### Decision de arquitectura de vistas

- Se usara un enfoque mixto:
  - rutas/paginas separadas por dominio y contexto de uso
  - conditional rendering solo para acciones puntuales dentro de cada pagina
- No se usara una sola pagina gigante con condicionales por rol.

### Rutas objetivo (MVP reconstruido)

- `/(protected)/join`
  - acceso: `student`, `assistant`, `func`, `admin`
  - uso: unirse por codigo, iniciar intento, responder, enviar
- `/(protected)/results`
  - acceso: `student`, `assistant`, `func`, `admin`
  - uso: ver resultado por join code o por attempt
- `/(protected)/courses`
  - acceso: `assistant`, `func`, `admin`
  - uso: listar/crear cursos
- `/(protected)/courses/[courseId]/members`
  - acceso: `assistant`, `func`, `admin`
  - policy backend: solo course manager (`assistant|func`) o `admin`
- `/(protected)/courses/[courseId]/banks`
  - acceso: `assistant`, `func`, `admin`
  - uso: CRUD de bancos segun policy de curso
- `/(protected)/courses/[courseId]/quizzes`
  - acceso: `assistant`, `func`, `admin`
  - uso: crear/listar/eliminar quizzes
- `/(protected)/courses/[courseId]/quizzes/[quizId]/attempts`
  - acceso: `assistant`, `func`, `admin`
  - policy backend: list de intentos requiere manager member o admin
- `/(protected)/admin/users`
  - acceso: `func`, `admin`
  - uso: gestionar roles assistant globales

### Reglas de render condicional dentro de pagina

- `assistant`:
  - puede gestionar members en cursos donde sea manager member.
  - no puede eliminar curso.
- `func`:
  - todo lo de assistant.
  - puede eliminar curso.
  - puede promover student -> assistant global.
- `admin`:
  - acceso total.
- `student`:
  - solo flujos join/attempt/results.

### Matriz endpoint -> pagina (fuente de verdad frontend)

- `/courses` (GET/POST) -> `/(protected)/courses`
- `/courses/{courseId}` (GET/DELETE) -> `/(protected)/courses/[courseId]` y vistas derivadas
- `/courses/{courseId}/members` (GET/POST) -> `/(protected)/courses/[courseId]/members`
- `/courses/{courseId}/members/{userId}` (DELETE) -> `/(protected)/courses/[courseId]/members`
- `/banks/course/{courseId}` + `/banks/*` -> `/(protected)/courses/[courseId]/banks`
- `/quizzes/me`, `/quizzes`, `/quizzes/{quizId}` -> `/(protected)/courses/[courseId]/quizzes`
- `/attempts/course/{courseId}/quiz/{quizId}` (GET) -> `/(protected)/courses/[courseId]/quizzes/[quizId]/attempts`
- `/quizzes/join/{joinCode}` + `/attempts/*` submit/results -> `/(protected)/join` y `/(protected)/results`

### Nota de seguridad UX

- La UI debe ocultar acciones segun rol/contexto para mejor experiencia.
- Aun asi, el backend sigue siendo la fuente de verdad; hay que manejar `403` con feedback claro.
