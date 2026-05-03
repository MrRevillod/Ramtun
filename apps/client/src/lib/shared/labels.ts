import type { QuizKind } from "$lib/quizzes/types"
import type { CertaintyLevel } from "$lib/attempts/types"
import type { User } from "$lib/auth/types"
import type { CourseMemberRole } from "$lib/courses/types"

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
