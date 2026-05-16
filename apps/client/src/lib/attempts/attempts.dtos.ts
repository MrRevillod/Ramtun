import * as v from "valibot"

export const joinCodeSchema = v.object({
	joinCode: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(4, (info) =>
			info.input === ""
				? "Ingresa un código."
				: "El código es demasiado corto."
		),
		v.maxLength(32, "El código es demasiado largo.")
	),
})

export type JoinCodeFormValues = v.InferInput<typeof joinCodeSchema>

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
	preview: import("$lib/quizzes/quizzes.dtos").JoinQuizPreview
	attempt: AttemptView
	answers: Record<string, AnswerState>
	index: number
}
