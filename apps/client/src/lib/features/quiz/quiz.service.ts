import { request } from '$lib/shared/http/http'
import type { PromiseResult } from '$lib/shared/result'
import type { AppError } from '$lib/shared/errors'
import type {
	CreateQuizPayload,
	JoinQuizPayload,
	JoinQuizPreview,
	AttemptSnapshot,
	QuizDetail,
	QuizSummary
} from '$lib/features/quiz/types'
import type { ManagedUser } from '$lib/features/users/types'

class QuizService {
	createQuiz = async (
		payload: CreateQuizPayload
	): PromiseResult<QuizDetail, AppError> =>
		request<QuizDetail>({
			method: 'POST',
			url: '/quizzes',
			data: payload
		})

	joinQuizPreview = async (
		payload: JoinQuizPayload
	): PromiseResult<JoinQuizPreview, AppError> =>
		request<JoinQuizPreview>({
			method: 'POST',
			url: '/quizzes/join-by-code',
			data: payload
		})

	getMyQuizzes = async (): PromiseResult<QuizSummary[], AppError> =>
		request<QuizSummary[]>({
			method: 'GET',
			url: '/quizzes/me'
		})

	startAttempt = async (quizId: string): PromiseResult<AttemptSnapshot, AppError> =>
		request<AttemptSnapshot>({
			method: 'POST',
			url: `/quizzes/${quizId}/attempts`,
			data: null
		})

	getMyActiveAttempt = async (quizId: string): PromiseResult<AttemptSnapshot, AppError> =>
		request<AttemptSnapshot>({
			method: 'GET',
			url: `/quizzes/${quizId}/attempts/me`
		})

	getCollaborators = async (quizId: string): PromiseResult<ManagedUser[], AppError> =>
		request<ManagedUser[]>({
			method: 'GET',
			url: `/quizzes/${quizId}/collaborators`
		})

	addCollaborator = async (quizId: string, userId: string): PromiseResult<void, AppError> =>
		request<void>({
			method: 'PUT',
			url: `/quizzes/${quizId}/collaborators/${userId}`,
			data: null
		})

	removeCollaborator = async (quizId: string, userId: string): PromiseResult<void, AppError> =>
		request<void>({
			method: 'DELETE',
			url: `/quizzes/${quizId}/collaborators/${userId}`
		})
}

export const quizService = new QuizService()
