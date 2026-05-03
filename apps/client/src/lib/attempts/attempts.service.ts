import { request } from "$lib/shared/http/http"
import { unwrapResultOrThrow, type AppResultAsync } from "$lib/shared/result"
import type {
	AttemptListItem,
	AttemptResult,
	AttemptSubmitView,
	AttemptView,
	SaveAnswerInput,
} from "$lib/attempts/types"

const COURSE_ID_PLACEHOLDER = "00000000-0000-0000-0000-000000000000"

class AttemptsService {
	public listAttempts(
		courseId: string,
		quizId: string
	): AppResultAsync<AttemptListItem[]> {
		return request<AttemptListItem[]>({
			method: "GET",
			url: `/attempts/course/${courseId}/quiz/${quizId}`,
		})
	}

	public async listAttemptsOrThrow(
		courseId: string,
		quizId: string
	): Promise<AttemptListItem[]> {
		return unwrapResultOrThrow(await this.listAttempts(courseId, quizId))
	}

	public initialize(quizId: string): AppResultAsync<AttemptView> {
		return request<AttemptView>({
			method: "POST",
			url: `/attempts/course/${COURSE_ID_PLACEHOLDER}/quiz/${quizId}`,
		})
	}

	public async initializeOrThrow(quizId: string): Promise<AttemptView> {
		return unwrapResultOrThrow(await this.initialize(quizId))
	}

	public saveAnswer(
		attemptId: string,
		questionId: string,
		input: SaveAnswerInput
	): AppResultAsync<void> {
		return request<void>({
			method: "PUT",
			url: `/attempts/${attemptId}/answers/${questionId}`,
			data: input,
		})
	}

	public async saveAnswerOrThrow(
		attemptId: string,
		questionId: string,
		input: SaveAnswerInput
	): Promise<void> {
		return unwrapResultOrThrow(await this.saveAnswer(attemptId, questionId, input))
	}

	public submit(attemptId: string): AppResultAsync<AttemptSubmitView> {
		return request<AttemptSubmitView>({
			method: "POST",
			url: `/attempts/${attemptId}/submit`,
		})
	}

	public async submitOrThrow(attemptId: string): Promise<AttemptSubmitView> {
		return unwrapResultOrThrow(await this.submit(attemptId))
	}

	public getResults(attemptId: string): AppResultAsync<AttemptResult> {
		return request<AttemptResult>({
			method: "GET",
			url: `/attempts/${attemptId}/results`,
		})
	}

	public async getResultsOrThrow(attemptId: string): Promise<AttemptResult> {
		return unwrapResultOrThrow(await this.getResults(attemptId))
	}

	public getAttemptResultsManaged(attemptId: string): AppResultAsync<AttemptResult> {
		return request<AttemptResult>({
			method: "GET",
			url: `/attempts/${attemptId}/results/managed`,
		})
	}

	public async getAttemptResultsManagedOrThrow(
		attemptId: string
	): Promise<AttemptResult> {
		return unwrapResultOrThrow(await this.getAttemptResultsManaged(attemptId))
	}
}

export const attemptsService = new AttemptsService()
