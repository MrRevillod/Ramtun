import { quizzesService } from "$lib/quizzes/quizzes.service"
import type { LayoutLoad } from "./$types"

export const load: LayoutLoad = async ({ params }) => {
	const quiz = await quizzesService.getQuiz(params.quizId)
	return {
		courseId: params.courseId,
		quizId: params.quizId,
		attemptId: params.attemptId,
		quizName: quiz.title,
	}
}
