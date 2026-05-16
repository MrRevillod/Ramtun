import * as v from "valibot"

export const createQuizSchema = v.object({
	title: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El titulo es obligatorio."),
		v.maxLength(100, "Maximo 100 caracteres.")
	),
	kind: v.picklist(["traditional", "certainty"]),
	startsAt: v.pipe(v.string(), v.minLength(1, "La fecha de inicio es obligatoria.")),
	attemptDurationMinutes: v.pipe(
		v.string(),
		v.toNumber(),
		v.minValue(1, "Minimo 1 minuto."),
		v.maxValue(240, "Maximo 240 minutos.")
	),
	questionCount: v.pipe(
		v.string(),
		v.toNumber(),
		v.minValue(1, "Minimo 1 pregunta."),
		v.maxValue(100, "Maximo 100 preguntas.")
	),
})

export type CreateQuizFormValues = v.InferInput<typeof createQuizSchema>

export type QuizKind = "traditional" | "certainty"

export type CertaintyScore = {
	correct: number
	incorrect: number
}

export type CertaintyTable = {
	low: CertaintyScore
	medium: CertaintyScore
	high: CertaintyScore
}

export type QuizCourseSummary = {
	id: string
	name: string
	code: string
	year: number
}

export type Quiz = {
	id: string
	title: string
	kind: QuizKind
	joinCode: string
	questionCount: number
	certaintyTable: CertaintyTable | null
	attemptDurationMinutes: number
	startsAt: string
	closedAt: string | null
	resultsPublishedAt: string | null
	createdAt: string
	course: QuizCourseSummary
}

export type CreateQuizInput = {
	courseId: string
	title: string
	kind: QuizKind
	startsAt: string
	attemptDurationMinutes: number
	questionCount: number
	bankIds: string[]
	certaintyConfig: CertaintyTable | null
}

export type JoinQuizPreview = {
	quizId: string
	title: string
	kind: QuizKind
	questionCount: number
	certaintyTable: CertaintyTable | null
	attemptDurationMinutes: number
	startsAt: string
}
