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

export type BankUploadValues = v.InferInput<typeof bankUploadSchema>
export type QuestionInput = v.InferInput<typeof questionInputSchema>
