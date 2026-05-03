import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ params }) => {
	return {
		courseId: params.courseId,
		quizId: params.quizId,
	}
}
