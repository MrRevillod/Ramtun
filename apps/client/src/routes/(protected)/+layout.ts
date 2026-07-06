import { redirect } from "@sveltejs/kit"
import { authService } from "$lib/auth/auth.service"
import { authStore } from "$lib/auth/auth.store.svelte"

export const load = async () => {
	try {
		await authService.bootstrapSession()
	} catch {
		authStore.clearSession()
	}

	if (!authStore.isAuthenticated()) {
		throw redirect(302, "/login")
	}
}
