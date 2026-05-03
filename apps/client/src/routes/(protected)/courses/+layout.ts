import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/auth.store.svelte"
import { isTeachingRole } from "$lib/shared/auth/permissions"

export const load = async () => {
	if (!isTeachingRole(authStore.session?.user.role)) {
		throw redirect(302, "/join")
	}
}
