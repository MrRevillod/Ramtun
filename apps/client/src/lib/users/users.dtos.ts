export type ManagedUserRole = "student" | "assistant" | "func" | "admin"

export type ManagedUser = {
	id: string
	username: string
	name: string
	email: string
	role: ManagedUserRole
}
