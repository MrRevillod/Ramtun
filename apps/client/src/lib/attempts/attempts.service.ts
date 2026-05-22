import { request } from "$lib/shared/http/http"
import type {
	AttemptListItem,
	AttemptResult,
	AttemptSubmitView,
	AttemptView,
	SaveAnswerInput,
} from "$lib/attempts/attempts.dtos"

class AttemptsService {
	public listAttempts(courseId: string, quizId: string): Promise<AttemptListItem[]> {
		return request<AttemptListItem[]>({
			method: "GET",
			url: `/attempts/course/${courseId}/quiz/${quizId}`,
		})
	}

	public initialize(quizId: string): Promise<AttemptView> {
		return request<AttemptView>({
			method: "POST",
			url: `/attempts/quiz/${quizId}`,
		})
	}

	public saveAnswer(
		attemptId: string,
		questionId: string,
		input: SaveAnswerInput
	): Promise<void> {
		return request<void>({
			method: "PUT",
			url: `/attempts/${attemptId}/answers/${questionId}`,
			data: {
				...input,
				questionId,
				attemptId,
			},
		})
	}

	public submit(attemptId: string): Promise<AttemptSubmitView> {
		return request<AttemptSubmitView>({
			method: "POST",
			url: `/attempts/${attemptId}/submit`,
		})
	}

	public getResultsByJoinCode(joinCode: string): Promise<AttemptResult> {
		return request<AttemptResult>({
			method: "GET",
			url: `/attempts/join/${joinCode}/results/me`,
		})
	}

	public getAttemptResultsManaged(attemptId: string): Promise<AttemptResult> {
		return request<AttemptResult>({
			method: "GET",
			url: `/attempts/${attemptId}/results/managed`,
		})
	}
}

export const attemptsService = new AttemptsService()
