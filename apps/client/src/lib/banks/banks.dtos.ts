import * as v from "valibot"

const questionInputSchema = v.object({
	prompt: v.pipe(
		v.string(),
		v.minLength(1, "Prompt debe tener entre 1 y 1000 caracteres."),
		v.maxLength(1000, "Prompt debe tener entre 1 y 1000 caracteres.")
	),
	options: v.pipe(
		v.array(v.string()),
		v.minLength(2, "Debe haber entre 2 y 5 opciones."),
		v.maxLength(5, "Debe haber entre 2 y 5 opciones.")
	),
	answerIndex: v.pipe(
		v.number(),
		v.integer("answerIndex debe ser un entero válido."),
		v.minValue(0, "answerIndex no puede ser negativo.")
	),
	images: v.pipe(
		v.array(v.string()),
		v.maxLength(5, "No puede haber más de 5 imágenes por pregunta.")
	),
})

const questionSchema = v.pipe(
	questionInputSchema,
	v.check(
		q => q.answerIndex < q.options.length,
		"answerIndex debe ser un índice válido de las opciones."
	)
)

export const bankUploadSchema = v.object({
	name: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El nombre es obligatorio."),
		v.maxLength(120, "Máximo 120 caracteres.")
	),
	questions: v.pipe(
		v.array(questionSchema),
		v.minLength(1, "Debe haber al menos 1 pregunta."),
		v.maxLength(500, "Debe haber entre 1 y 500 preguntas.")
	),
})

export const bankQuestionsSchema = v.pipe(
	v.array(questionSchema),
	v.minLength(1, "Debe haber al menos 1 pregunta."),
	v.maxLength(500, "Debe haber entre 1 y 500 preguntas.")
)

export const newQuestionSchema = v.object({
	prompt: v.string(),
	options: v.array(v.string()),
	answerIndex: v.number(),
	images: v.optional(v.array(v.string())),
})

export const legacyQuestionSchema = v.object({
	question: v.string(),
	options: v.array(v.string()),
	answer: v.number(),
	images: v.optional(v.array(v.string())),
})

export const legacyBankSchema = v.object({
	questions: v.array(legacyQuestionSchema),
})

export type BankUploadValues = v.InferInput<typeof bankUploadSchema>
export type QuestionInput = v.InferInput<typeof questionInputSchema>
export type NewQuestion = v.InferInput<typeof newQuestionSchema>
export type LegacyQuestion = v.InferInput<typeof legacyQuestionSchema>

export const normalizeQuestions = (parsed: unknown): QuestionInput[] => {
	const newResult = v.safeParse(v.array(newQuestionSchema), parsed)
	if (newResult.success) {
		return newResult.output.map((q: NewQuestion) => ({
			prompt: q.prompt,
			options: q.options,
			answerIndex: q.answerIndex,
			images: q.images ?? [],
		}))
	}

	const legacyResult = v.safeParse(legacyBankSchema, parsed)
	if (legacyResult.success) {
		return legacyResult.output.questions.map((q: LegacyQuestion) => ({
			prompt: q.question,
			options: q.options,
			answerIndex: q.answer,
			images: q.images ?? [],
		}))
	}

	throw new Error(
		"El JSON debe ser un array de preguntas o un objeto con 'questions'."
	)
}

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
