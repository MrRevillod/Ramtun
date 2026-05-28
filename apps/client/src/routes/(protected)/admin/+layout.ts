import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/auth.store.svelte"
import { isAdmin } from "$lib/shared/auth/permissions"

export const load = async () => {
	if (!isAdmin(authStore.session?.user.role)) {
		throw redirect(302, "/join")
	}
}
