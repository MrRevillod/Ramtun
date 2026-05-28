import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/auth.store.svelte"
import { isAdmin, isFunc } from "$lib/shared/auth/permissions"
import { coursesService } from "$lib/courses/courses.service"

export const load = async () => {
	const role = authStore.session?.user.role

	if (isAdmin(role) || isFunc(role)) {
		throw redirect(302, "/courses")
	}

	const courses = await coursesService.list()
	if (courses.length > 0) {
		throw redirect(302, "/courses")
	}

	throw redirect(302, "/join")
}
