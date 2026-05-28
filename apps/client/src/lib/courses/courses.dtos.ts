import * as v from "valibot"

export const createCourseSchema = v.object({
	name: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El nombre es obligatorio."),
		v.maxLength(120, "Maximo 120 caracteres.")
	),
	code: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(2, "Minimo 2 caracteres."),
		v.maxLength(32, "Maximo 32 caracteres.")
	),
	year: v.pipe(
		v.number("El año debe ser numerico."),
		v.minValue(2000, "El año minimo es 2000."),
		v.maxValue(2100, "El año maximo es 2100.")
	),
})

export const addCourseMemberSchema = v.object({
	userId: v.pipe(v.string(), v.minLength(1, "Debe seleccionar un usuario.")),
})

export type CreateCourseFormValues = v.InferInput<typeof createCourseSchema>
export type AddCourseMemberFormValues = v.InferInput<typeof addCourseMemberSchema>

export type Course = {
	id: string
	name: string
	code: string
	year: number
}

export type CourseMemberRole = "assistant" | "func"

export type CourseMember = {
	userId: string
	username: string
	name: string
	role: CourseMemberRole
}

export type CourseView = {
	id: string
	name: string
	code: string
	year: number
	members: CourseMember[]
}

export type CreateCourseInput = {
	name: string
	code: string
	year: number
}

export type AddCourseMemberInput = {
	userId: string
}
