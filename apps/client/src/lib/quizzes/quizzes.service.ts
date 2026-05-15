import { request } from "$lib/shared/http/http"
import { unwrapResultOrThrow, type AppResultAsync } from "$lib/shared/result"
import type { CreateQuizInput, JoinQuizPreview, Quiz } from "$lib/quizzes/types"

class QuizzesService {
	public listManaged(): AppResultAsync<Quiz[]> {
		return request<Quiz[]>({
			method: "GET",
			url: "/quizzes/me",
		})
	}

	public async listManagedOrThrow(): Promise<Quiz[]> {
		return unwrapResultOrThrow(await this.listManaged())
	}

	public create(input: CreateQuizInput): AppResultAsync<Quiz> {
		return request<Quiz>({
			method: "POST",
			url: "/quizzes",
			data: input,
		})
	}

	public async createOrThrow(input: CreateQuizInput): Promise<Quiz> {
		return unwrapResultOrThrow(await this.create(input))
	}

	public remove(quizId: string): AppResultAsync<void> {
		return request<void>({
			method: "DELETE",
			url: `/quizzes/${quizId}`,
		})
	}

	public async removeOrThrow(quizId: string): Promise<void> {
		return unwrapResultOrThrow(await this.remove(quizId))
	}

	public closeAndPublish(quizId: string): AppResultAsync<void> {
		return request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/close-and-publish`,
		})
	}

	public async closeAndPublishOrThrow(quizId: string): Promise<void> {
		return unwrapResultOrThrow(await this.closeAndPublish(quizId))
	}

	public joinByCode(joinCode: string): AppResultAsync<JoinQuizPreview> {
		return request<JoinQuizPreview>({
			method: "POST",
			url: `/quizzes/join/${joinCode}`,
		})
	}

	public async joinByCodeOrThrow(joinCode: string): Promise<JoinQuizPreview> {
		return unwrapResultOrThrow(await this.joinByCode(joinCode))
	}
}

export const quizzesService = new QuizzesService()
