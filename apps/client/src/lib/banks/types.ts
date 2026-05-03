export type Question = {
	id: string
	prompt: string
	options: string[]
	answerIndex: number
	images: string[]
}

export type QuestionBank = {
	id: string
	courseId: string
	name: string
	questions: Question[]
	createdAt: string
	deletedAt: string | null
}

export type CreateQuestionBankInput = {
	courseId: string
	name: string
	questions: {
		prompt: string
		options: string[]
		answerIndex: number
		images: string[]
	}[]
}
