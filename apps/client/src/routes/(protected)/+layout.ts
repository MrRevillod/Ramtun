import type { LayoutLoad } from "./$types"

import { redirect } from "@sveltejs/kit"
import { inlineTryAsync } from "$lib/shared/try"

import { authStore } from "$lib/auth/store.svelte"
import { authService } from "$lib/auth/service"
import { coursesService } from "$lib/courses/service"

export const load: LayoutLoad = async () => {
	const [_, err] = await inlineTryAsync(() => authService.bootstrapSession())

	if (err) {
		authStore.clearSession()
	}

	if (!authStore.isAuthenticated()) {
		redirect(302, "/login")
	}

	const hasCourses = await coursesService.list().then((courses) => courses.length > 0)

	return {
		hasCourses,
	}
}
