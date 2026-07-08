import { redirect } from "@sveltejs/kit"
import { coursesService } from "$lib/courses/service"
import { inlineTryAsync } from "$lib/shared/try"

export const load = async ({ params }) => {
	const [course, err] = await inlineTryAsync(() => coursesService.findOne(params.courseId))

	if (err) {
		throw redirect(307, "/courses")
	}

	return { courseId: params.courseId, course: course! }
}
