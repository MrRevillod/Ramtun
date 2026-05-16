import type { QuizKind } from "$lib/quizzes/quizzes.dtos"
import type { CertaintyLevel } from "$lib/attempts/attempts.dtos"
import type { User } from "$lib/auth/auth.dtos"
import type { CourseMemberRole } from "$lib/courses/courses.dtos"

type AnyRole = User["role"] | CourseMemberRole

const quizKindLabels: Record<QuizKind, string> = {
	traditional: "Tradicional",
	certainty: "Certeza",
}

const certaintyLabels: Record<CertaintyLevel, string> = {
	low: "Baja",
	medium: "Media",
	high: "Alta",
}

const roleLabels: Record<AnyRole, string> = {
	student: "Estudiante",
	assistant: "Ayudante",
	func: "Profesor",
	admin: "Administrador",
}

export const quizKindLabel = (kind: QuizKind): string => quizKindLabels[kind]
export const certaintyLevelLabel = (level: CertaintyLevel): string =>
	certaintyLabels[level]
export const roleLabel = (role: AnyRole): string => roleLabels[role]
