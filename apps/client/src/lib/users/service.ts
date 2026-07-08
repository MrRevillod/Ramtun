import type { UserDTO, UserRoleDTO } from "./dtos"

import { User } from "$lib/users/entity"
import { http } from "$lib/shared/http/client"

class UsersService {
	public listUsers(params?: { search?: string; roles?: string }): Promise<User[]> {
		const users = http.request<UserDTO[]>({
			method: "GET",
			url: "/users",
			params: { ...params },
		})

		return users.then((users) => users.map(User.fromDTO))
	}

	public updateRole(userId: string, role: UserRoleDTO): Promise<unknown> {
		return http.request<unknown>({
			method: "PATCH",
			url: `/users/${userId}/role`,
			data: { role },
		})
	}

	public listCollaboratorCandidates(query?: string): Promise<User[]> {
		const users = http.request<UserDTO[]>({
			method: "GET",
			url: "/users/collaborator-candidates",
			params: {
				...(query ? { search: query } : {}),
			},
		})

		return users.then((users) => users.map(User.fromDTO))
	}
}

export const usersService = new UsersService()
