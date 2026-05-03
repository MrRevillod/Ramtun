import { request } from "$lib/shared/http/http"
import { unwrapResultOrThrow, type AppResultAsync } from "$lib/shared/result"
import type { CreateQuizInput, JoinQuizPreview, Quiz } from "$lib/quizzes/types"
import type { AttemptResult } from "$lib/attempts/types"

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

	public close(quizId: string): AppResultAsync<void> {
		return request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/close`,
		})
	}

	public async closeOrThrow(quizId: string): Promise<void> {
		return unwrapResultOrThrow(await this.close(quizId))
	}

	public publishResults(quizId: string): AppResultAsync<void> {
		return request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/publish-results`,
		})
	}

	public async publishResultsOrThrow(quizId: string): Promise<void> {
		return unwrapResultOrThrow(await this.publishResults(quizId))
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

	public getMyResultByCode(joinCode: string): AppResultAsync<AttemptResult> {
		return request<AttemptResult>({
			method: "GET",
			url: `/quizzes/join/${joinCode}/attempts/me/result`,
		})
	}

	public async getMyResultByCodeOrThrow(joinCode: string): Promise<AttemptResult> {
		return unwrapResultOrThrow(await this.getMyResultByCode(joinCode))
	}
}

export const quizzesService = new QuizzesService()
