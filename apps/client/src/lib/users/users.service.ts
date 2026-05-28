import type { ManagedUser, ManagedUserRole } from "$lib/users/users.dtos"
import { request } from "$lib/shared/http/http"

class UsersService {
	public listUsers(params?: {
		search?: string
		roles?: string
	}): Promise<ManagedUser[]> {
		return request<ManagedUser[]>({
			method: "GET",
			url: "/users",
			params: { ...params },
		})
	}

	public updateRole(userId: string, role: ManagedUserRole): Promise<ManagedUser> {
		return request<ManagedUser>({
			method: "PATCH",
			url: `/users/${userId}/role`,
			data: { role },
		})
	}

	public listCollaboratorCandidates(query?: string): Promise<ManagedUser[]> {
		return request<ManagedUser[]>({
			method: "GET",
			url: "/users/collaborator-candidates",
			params: {
				...(query ? { search: query } : {}),
			},
		})
	}
}

export const usersService = new UsersService()
