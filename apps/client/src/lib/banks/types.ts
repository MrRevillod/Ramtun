export type Question = {
	id: string
	prompt: string
	options: string[]
	answer_index?: number
	answerIndex?: number
	images: string[]
}

export type QuestionBank = {
	id: string
	course_id?: string
	courseId?: string
	name: string
	questions: Question[]
	created_at?: string
	createdAt?: string
	deleted_at?: string | null
	deletedAt?: string | null
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
