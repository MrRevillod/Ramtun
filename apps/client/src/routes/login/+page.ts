import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/store.svelte"
import { authService } from "$lib/auth/service"

export const load = async () => {
	await authService.bootstrapSession()

	if (authStore.isAuthenticated()) {
		throw redirect(302, "/")
	}
}
