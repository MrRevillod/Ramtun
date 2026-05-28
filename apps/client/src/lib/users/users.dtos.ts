export type ManagedUserRole = "student" | "func"

export type ManagedUser = {
	id: string
	username: string
	name: string
	email: string
	role: "student" | "func" | "admin"
}
