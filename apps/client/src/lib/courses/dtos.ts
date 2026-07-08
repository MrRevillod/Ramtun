import * as v from "valibot"

// Course View DTOs -------------------------------------------

export interface CourseMemberDTO {
	userId: string
	username: string
	name: string
	role: "assistant" | "func"
}

export interface CourseDTO {
	id: string
	name: string
	code: string
	year: number
	members: CourseMemberDTO[]
}

// CreateCourseDTO --------------------------------------------

export const createCourseDTOSchema = v.object({
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

export type CreateCourseDTOSchema = typeof createCourseDTOSchema
export type CreateCourseDTO = v.InferInput<typeof createCourseDTOSchema>

// AddCourseMemberDTO --------------------------------------------

export const addCourseMemberDTOSchema = v.object({
	userId: v.pipe(v.string(), v.minLength(1, "Debe seleccionar un usuario.")),
})

export type AddCourseMemberDTOSchema = typeof addCourseMemberDTOSchema
export type AddCourseMemberDTO = v.InferInput<typeof addCourseMemberDTOSchema>
