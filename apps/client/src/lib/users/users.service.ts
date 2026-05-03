import { request } from "$lib/shared/http/http"
import { unwrapResultOrThrow, type AppResultAsync } from "$lib/shared/result"
import type { ManagedUser } from "$lib/users/types"

class UsersService {
	public listCollaboratorCandidates(query?: string): AppResultAsync<ManagedUser[]> {
		return request<ManagedUser[]>({
			method: "GET",
			url: "/users/collaborator-candidates",
			params: {
				...(query ? { search: query } : {}),
			},
		})
	}

	public async listCollaboratorCandidatesOrThrow(
		query?: string
	): Promise<ManagedUser[]> {
		return unwrapResultOrThrow(await this.listCollaboratorCandidates(query))
	}
}

export const usersService = new UsersService()
