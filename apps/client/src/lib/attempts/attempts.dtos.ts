import * as v from "valibot"
import type { JoinQuizPreview } from "$lib/quizzes/quizzes.dtos"

export const joinCodeFormSchema = v.object({
	joinCode: v.nonOptional(
		v.pipe(
			v.string(),
			v.trim(),
			v.minLength(4, info =>
				info.input === "" ? "Ingresa un código." : "El código es demasiado corto."
			),
			v.maxLength(32, "El código es demasiado largo.")
		),
		"El código es obligatorio."
	),
})

export type JoinCodeFormData = v.InferInput<typeof joinCodeFormSchema>

export type CertaintyLevel = "low" | "medium" | "high"

export type AttemptQuestion = {
	id: string
	prompt: string
	options: string[]
	images: string[]
}

export type AttemptView = {
	attemptId: string
	quizId: string
	startedAt: string
	expiresAt: string
	submittedAt: string | null
	resultsViewedAt: string | null
	questions: AttemptQuestion[]
}

export type AttemptListItem = {
	attemptId: string
	studentId: string
	userName: string
	quizId: string
	startedAt: string
	expiresAt: string
	submittedAt: string | null
	resultsViewedAt: string | null
	score: number | null
	grade: number | null
}

export type AttemptSubmitView = {
	attemptId: string
	quizId: string
	submittedAt: string
}

export type QuestionResult = {
	questionId: string
	question: string
	options: string[]
	images: string[]
	answerIndex: number | null
	correctAnswerIndex: number
	certaintyLevel: CertaintyLevel | null
	isCorrect: boolean
	awardedPoints: number
}

export type AttemptResult = {
	attemptId: string
	studentId: string
	userName: string
	quizId: string
	submittedAt: string
	grade: number
	score: number
	maxScore: number
	resultsViewedAt: string | null
	questions: QuestionResult[]
}

export type SaveAnswerInput = {
	answerIndex: number
	certaintyLevel: CertaintyLevel | null
	questionId: string
}

export type AnswerState = {
	answerIndex: number
	certaintyLevel: CertaintyLevel | null
}

export type AttemptSession = {
	joinCode: string
	preview: JoinQuizPreview
	attempt: AttemptView
	answers: Record<string, AnswerState>
	index: number
}

export type WarningType =
	| "focus_loss"
	| "clipboard"
	| "screenshot"
	| "navigation"
	| "devtools"

export type AttemptWarning = {
	id: string
	attemptId: string
	warningType: WarningType
	details: string
	sequenceNumber: number
	createdAt: string
}

export const WARNING_LABELS: Record<WarningType, string> = {
	focus_loss: "Pérdida de foco",
	clipboard: "Copia de texto",
	screenshot: "Captura de pantalla",
	navigation: "Navegación del sistema",
	devtools: "Herramientas de desarrollador",
}
