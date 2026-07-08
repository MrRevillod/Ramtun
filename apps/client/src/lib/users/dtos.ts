export type UserRoleDTO = "admin" | "func" | "student"
export type ManageableRoleDTO = Omit<UserRoleDTO, "admin">

export interface UserDTO {
	id: string
	username: string
	name: string
	email: string
	role: UserRoleDTO
}
