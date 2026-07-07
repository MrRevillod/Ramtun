import type {
	AttemptListItem,
	AttemptResult,
	AttemptSubmitView,
	AttemptView,
	AttemptWarning,
	SaveAnswerInput,
	WarningType,
} from "$lib/attempts/attempts.dtos"

import { http } from "$lib/shared/http/request"

class AttemptsService {
	public listAttempts(courseId: string, quizId: string): Promise<AttemptListItem[]> {
		return http.request<AttemptListItem[]>({
			method: "GET",
			url: `/attempts/course/${courseId}/quiz/${quizId}`,
		})
	}

	public getActive(): Promise<AttemptView> {
		return http.request<AttemptView>({
			method: "GET",
			url: "/attempts/me/active-attempt",
		})
	}

	public getById(attemptId: string): Promise<AttemptView> {
		return http.request<AttemptView>({
			method: "GET",
			url: `/attempts/${attemptId}`,
		})
	}

	public initialize(quizId: string): Promise<AttemptView> {
		return http.request<AttemptView>({
			method: "POST",
			url: `/attempts/quiz/${quizId}`,
		})
	}

	public saveAnswer(attemptId: string, questionId: string, input: SaveAnswerInput): Promise<void> {
		return http.request<void>({
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
		return http.request<AttemptSubmitView>({
			method: "POST",
			url: `/attempts/${attemptId}/submit`,
		})
	}

	public getResultsByJoinCode(joinCode: string): Promise<AttemptResult> {
		return http.request<AttemptResult>({
			method: "GET",
			url: `/attempts/join/${joinCode}/results/me`,
		})
	}

	public getAttemptResultsManaged(attemptId: string): Promise<AttemptResult> {
		return http.request<AttemptResult>({
			method: "GET",
			url: `/attempts/${attemptId}/results/managed`,
		})
	}

	public getAttemptWarnings(attemptId: string): Promise<AttemptWarning[]> {
		return http.request<AttemptWarning[]>({
			method: "GET",
			url: `/attempts/${attemptId}/warnings`,
		})
	}

	public recordWarning(
		attemptId: string,
		warningType: WarningType,
		details: string
	): Promise<AttemptWarning> {
		return http.request<AttemptWarning>({
			method: "POST",
			url: `/attempts/${attemptId}/warnings`,
			data: { warningType, details },
		})
	}

	public getQuizWarnings(quizId: string): Promise<AttemptWarning[]> {
		return http.request<AttemptWarning[]>({
			method: "GET",
			url: `/attempts/quiz/${quizId}/warnings`,
		})
	}
}

export const attemptsService = new AttemptsService()
