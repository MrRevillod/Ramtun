import { quizzesService } from "$lib/quizzes/service"
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ params }) => {
	const quiz = await quizzesService.findOne(params.quizId)

	return {
		courseId: params.courseId,
		quizId: params.quizId,
		quizName: quiz.title,
	}
}
