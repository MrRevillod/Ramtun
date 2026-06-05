import { redirect } from "@sveltejs/kit"
import { coursesService } from "$lib/courses/courses.service"

import type { CourseView } from "$lib/courses/courses.dtos"

export const load = async ({ params }) => {
	let course: CourseView | null = null

	try {
		course = await coursesService.get(params.courseId)
	} catch {
		throw redirect(307, "/courses")
	}

	return { courseId: params.courseId, course }
}
