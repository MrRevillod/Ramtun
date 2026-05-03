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
