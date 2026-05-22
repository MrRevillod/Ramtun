import type { ManagedUser } from "$lib/users/users.dtos"
import { request } from "$lib/shared/http/http"

class UsersService {
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
