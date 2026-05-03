import { redirect } from "@sveltejs/kit"
import { authService } from "$lib/auth/auth.service"
import { authStore } from "$lib/auth/auth.store.svelte"
import { sessionManager } from "$lib/shared/auth/session.manager"

export const load = async () => {
	const bootstrapResult = await authService.bootstrapSession()

	if (bootstrapResult.isErr()) {
		sessionManager.clearSession()
	}

	if (!authStore.isAuthenticated()) {
		throw redirect(302, "/login")
	}
}
