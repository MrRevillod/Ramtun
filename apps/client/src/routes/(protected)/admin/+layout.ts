import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/auth.store.svelte"
import { canManageUsers } from "$lib/shared/auth/permissions"

export const load = async () => {
	if (!canManageUsers(authStore.session?.user.role)) {
		throw redirect(302, "/join")
	}
}
