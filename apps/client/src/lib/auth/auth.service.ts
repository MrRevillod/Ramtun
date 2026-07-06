import { authStore } from "$lib/auth/auth.store.svelte"
import { request } from "$lib/shared/http/http"
import type { LoginInput, User } from "$lib/auth/auth.dtos"

class AuthService {
	private bootstrapPromise: Promise<void> | null = null

	public async login(input: LoginInput): Promise<User> {
		return request<User>({
			method: "POST",
			url: "/auth/login",
			data: input,
			skipRefresh: true,
		})
	}

	public async logout(): Promise<void> {
		return request<void>({ method: "POST", url: "/auth/logout" }).catch(() => {
			authStore.clearSession()
		})
	}

	public async bootstrapSession(): Promise<void> {
		if (authStore.isReady) return

		if (this.bootstrapPromise) return this.bootstrapPromise

		this.bootstrapPromise = (async () => {
			authStore.isBootstrapping = true

			try {
				const user = await request<User>({ method: "GET", url: "/users/me" })
				authStore.setSession(user)
			} catch {
				// Sin sesión — no es un error
			}

			authStore.isReady = true
			authStore.isBootstrapping = false
		})()

		const result = await this.bootstrapPromise
		this.bootstrapPromise = null
		return result
	}
}

export const authService = new AuthService()
