import { request } from "$lib/shared/http/http"
import type {
	CreateQuizInput,
	JoinQuizPreview,
	Quiz,
} from "$lib/quizzes/quizzes.dtos"

class QuizzesService {
	public listManaged(): Promise<Quiz[]> {
		return request<Quiz[]>({
			method: "GET",
			url: "/quizzes/me",
		})
	}

	public create(input: CreateQuizInput): Promise<Quiz> {
		return request<Quiz>({
			method: "POST",
			url: "/quizzes",
			data: input,
		})
	}

	public remove(quizId: string): Promise<void> {
		return request<void>({
			method: "DELETE",
			url: `/quizzes/${quizId}`,
		})
	}

	public closeAndPublish(quizId: string): Promise<void> {
		return request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/close-and-publish`,
		})
	}

	public joinByCode(joinCode: string): Promise<JoinQuizPreview> {
		return request<JoinQuizPreview>({
			method: "POST",
			url: `/quizzes/join/${joinCode}`,
		})
	}
}

export const quizzesService = new QuizzesService()
