import { request } from '$lib/shared/http/http'
import type { PromiseResult } from '$lib/shared/result'
import type { AppError } from '$lib/shared/errors'
import type { AttemptAnswer, AttemptCertaintyLevel, AttemptSnapshot } from '$lib/features/quiz/types'

export type SaveAttemptAnswerPayload = {
	answerIndex: number
	certaintyLevel?: AttemptCertaintyLevel | null
}

class AttemptService {
	saveAnswer = async (
		attemptId: string,
		questionId: string,
		payload: SaveAttemptAnswerPayload
	): PromiseResult<AttemptAnswer, AppError> =>
		request<AttemptAnswer>({
			method: 'PUT',
			url: `/attempts/${attemptId}/answers/${questionId}`,
			data: payload
		})

	submitAttempt = async (attemptId: string): PromiseResult<AttemptSnapshot, AppError> =>
		request<AttemptSnapshot>({
			method: 'POST',
			url: `/attempts/${attemptId}/submit`,
			data: null
		})
}

export const attemptService = new AttemptService()
