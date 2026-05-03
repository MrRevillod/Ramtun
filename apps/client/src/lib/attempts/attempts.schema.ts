import * as v from "valibot"

export const joinCodeSchema = v.object({
	joinCode: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(4, "El codigo es demasiado corto."),
		v.maxLength(32, "El codigo es demasiado largo.")
	),
})

export type JoinCodeFormValues = v.InferInput<typeof joinCodeSchema>
