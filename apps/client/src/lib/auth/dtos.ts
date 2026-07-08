import * as v from "valibot"

export const loginDTOSchema = v.object({
	username: v.pipe(v.string(), v.trim(), v.minLength(1, "El usuario es obligatorio.")),
	password: v.pipe(v.string(), v.minLength(1, "La contraseña es obligatoria.")),
})

export type LoginDTOSchema = typeof loginDTOSchema
export type LoginDTO = v.InferInput<typeof loginDTOSchema>
