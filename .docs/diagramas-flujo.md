# Diagramas de flujo — Ramtun

Diagramas de secuencia (alto nivel) de los dos flujos principales de la plataforma.
Los bloques Mermaid se renderizan automáticamente en GitHub.

## 1. Flujo Estudiante: entrar a un quiz, enviarlo y ver resultados

```mermaid
sequenceDiagram
    participant E as "Estudiante"
    participant APP as "Aplicación"
    participant SRV as "Servidor"

    E->>APP: Ingresa el código de acceso
    APP->>SRV: Solicita el quiz
    SRV-->>APP: Vista previa (título, tipo, duración)
    APP-->>E: Muestra la vista previa

    E->>APP: Inicia el intento
    APP->>SRV: Comienza el intento
    SRV->>SRV: Valida que pueda iniciar y prepara<br/>preguntas aleatorias con su tiempo
    SRV-->>APP: Intentó iniciado
    APP-->>E: Primera pregunta y temporizador

    loop Cada pregunta
        E->>APP: Elige alternativa y nivel de certeza
        APP->>SRV: Guarda la respuesta
        SRV-->>APP: Guardada
    end

    E->>APP: Envía el intento
    APP->>SRV: Envía el intento
    SRV->>SRV: Corrige, asigna puntos según la tabla<br/>de certeza y calcula la nota
    SRV-->>APP: Nota guardada
    APP-->>E: Confirmación de envío

    Note over E: Espera la publicación de resultados

    E->>APP: Consulta sus resultados
    APP->>SRV: Solicita los resultados
    SRV->>SRV: Verifica que estén publicados
    SRV-->>APP: Entrega nota y detalle por pregunta,<br/>con vista única
    APP-->>E: Muestra la nota final y la revisión
```

## 2. Flujo Docente/Ayudante: crear un quiz, monitorearlo y terminarlo

```mermaid
sequenceDiagram
    participant D as "Docente / Ayudante"
    participant APP as "Aplicación"
    participant SRV as "Servidor"

    D->>APP: Crea un banco de preguntas
    APP->>SRV: Guarda el banco
    SRV-->>APP: Banco creado

    D->>APP: Crea el quiz (bancos, tipo, duración, puntos)
    APP->>SRV: Crea el quiz
    SRV->>SRV: Valida, genera el código de acceso<br/>y copia las preguntas (inmutables)
    SRV-->>APP: Quiz creado con su código
    APP-->>D: Muestra el código de acceso
    D->>D: Comparte el código con los estudiantes

    Note over D,SRV: Los estudiantes inician y envían intentos
    SRV-->>APP: Avisa en vivo de cada intento y advertencia
    APP-->>D: Panel de intentos actualizado

    D->>APP: Revisa intentos y advertencias
    APP->>SRV: Obtiene intentos y advertencias
    SRV-->>APP: Intentos y advertencias
    APP-->>D: Los muestra

    Note over D: Mientras el quiz está abierto,<br/>el banco ligado queda bloqueado

    D->>APP: Cierra el quiz y publica resultados
    APP->>SRV: Finaliza y publica
    SRV->>SRV: Verifica permisos y publica
    SRV-->>APP: Resultados publicados
    APP-->>D: Confirmación
```
