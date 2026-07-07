import * as v from "valibot"

export type User = {
	id: string
	username: string
	name: string
	email: string
	role: "student" | "func" | "admin"
}

export const loginSchema = v.object({
	username: v.pipe(v.string(), v.trim(), v.minLength(1, "El usuario es obligatorio.")),
	password: v.pipe(v.string(), v.minLength(1, "La contraseña es obligatoria.")),
})

export type LoginInput = v.InferInput<typeof loginSchema>
