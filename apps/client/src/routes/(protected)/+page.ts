import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/auth.store.svelte"
import { isTeachingRole } from "$lib/shared/auth/permissions"

export const load = async () => {
	const role = authStore.session?.user.role

	if (isTeachingRole(role)) {
		throw redirect(302, "/courses")
	}

	throw redirect(302, "/join")
}
