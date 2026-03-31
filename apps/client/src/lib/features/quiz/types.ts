export type QuizMode = 'traditional' | 'certainty'

export type QuizKind = 'Traditional' | 'Certainly'

export type CertaintyWeight = {
	correct: number
	incorrect: number
}

export type CertaintyConfig = {
	low: CertaintyWeight
	medium: CertaintyWeight
	high: CertaintyWeight
}

export type QuizQuestion = {
	question: string
	options: string[]
	answer: number
	images: string[]
}

export type CreateQuizPayload = {
	title: string
	mode: QuizMode
	startTimeUtc: string
	attemptDurationMinutes: number
	collaboratorIds: string[]
	questions: QuizQuestion[]
	certaintyConfig: CertaintyConfig | null
}

export type JoinQuizPayload = {
	code: string
}

export type QuizSummary = {
	id: string
	title: string
	kind: QuizKind
	joinCode: string
	questionCount: number
	startTime: string
	attemptDurationMinutes: number
	createdAt: string
}

export type QuizDetailQuestion = {
	id: string
	question: string
	options: string[]
	answer: number
	images: string[]
}

export type QuizDetail = {
	id: string
	ownerId: string
	title: string
	kind: QuizKind
	joinCode: string
	questions: QuizDetailQuestion[]
	certaintyTable: CertaintyConfig | null
	certainlyTable?: CertaintyConfig | null
	startTime: string
	attemptDurationMinutes: number
	createdAt: string
	updatedAt: string
}

export type JoinQuizPreview = {
	id: string
	title: string
	kind: QuizKind
	questionCount: number
	certaintyTable: CertaintyConfig | null
	certainlyTable?: CertaintyConfig | null
	startTime: string
	attemptDurationMinutes: number
}

export type QuizParticipantQuestion = {
	questionId: string
	question: string
	options: string[]
	images: string[]
}

export type QuizParticipant = {
	id: string
	title: string
	kind: QuizKind
	questions: QuizParticipantQuestion[]
	certaintyTable: CertaintyConfig | null
	certainlyTable?: CertaintyConfig | null
	startTime: string
	attemptDurationMinutes: number
}

export type AttemptStatus = 'in_progress' | 'submitted'

export type AttemptCertaintyLevel = 'low' | 'medium' | 'high'

export type AttemptAnswer = {
	questionId: string
	answerIndex: number
	certaintyLevel: AttemptCertaintyLevel | null
}

export type AttemptSnapshot = {
	attemptId: string
	quizId: string
	startedAt: string
	expiresAt: string
	status: AttemptStatus
	quiz: QuizParticipant
	answers: AttemptAnswer[]
}
