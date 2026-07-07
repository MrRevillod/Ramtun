import { http } from "$lib/shared/http/request"
import type { CreateQuizInput, JoinQuizPreview, Quiz } from "$lib/quizzes/quizzes.dtos"

class QuizzesService {
	public listByCourse(courseId: string): Promise<Quiz[]> {
		return http.request<Quiz[]>({
			method: "GET",
			url: `/quizzes/course/${courseId}`,
		})
	}

	public create(input: CreateQuizInput): Promise<Quiz> {
		return http.request<Quiz>({
			method: "POST",
			url: "/quizzes",
			data: input,
		})
	}

	public remove(quizId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/quizzes/${quizId}`,
		})
	}

	public closeAndPublish(quizId: string): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/quizzes/${quizId}/close-and-publish`,
		})
	}

	public getQuiz(quizId: string): Promise<Quiz> {
		return http.request<Quiz>({
			method: "GET",
			url: `/quizzes/${quizId}`,
		})
	}

	public joinByCode(joinCode: string): Promise<JoinQuizPreview> {
		return http.request<JoinQuizPreview>({
			method: "POST",
			url: `/quizzes/join/${joinCode}`,
		})
	}
}

export const quizzesService = new QuizzesService()
