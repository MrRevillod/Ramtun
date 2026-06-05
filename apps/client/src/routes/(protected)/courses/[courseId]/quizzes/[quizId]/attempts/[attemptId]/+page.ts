import { redirect } from "@sveltejs/kit"
import type { PageLoad } from "./$types"

export const load: PageLoad = ({ params }) => {
	redirect(
		302,
		`/courses/${params.courseId}/quizzes/${params.quizId}/attempts/${params.attemptId}/results`
	)
}
