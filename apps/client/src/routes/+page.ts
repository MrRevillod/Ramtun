import { redirect } from "@sveltejs/kit"

import { authService } from "$lib/auth/service"
import { authStore } from "$lib/auth/store.svelte"
import { coursesService } from "$lib/courses/service"

export const load = async () => {
	await authService.bootstrapSession()

	const user = authStore.user
	if (!user) {
		throw redirect(302, "/login")
	}

	if (user.isAdmin() || user.isFunc()) {
		throw redirect(302, "/courses")
	}

	const courses = await coursesService.list()
	throw redirect(302, courses.length > 0 ? "/courses" : "/join")
}
