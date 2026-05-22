import { authStore } from "$lib/auth/auth.store.svelte"
import { sessionManager } from "$lib/shared/auth/session.manager"
import { request } from "$lib/shared/http/http"
import type { AuthTokens, LoginInput, LoginResponse } from "$lib/auth/auth.dtos"

class AuthService {
	#bootstrapPromise: Promise<void> | null = null

	public login(input: LoginInput): Promise<LoginResponse> {
		return request<LoginResponse>({
			method: "POST",
			url: "/auth/login",
			data: input,
			skipAuth: true,
			skipRefresh: true,
		})
	}

	public async refresh(): Promise<AuthTokens> {
		const refreshToken = sessionManager.getRefreshToken()

		if (!refreshToken) {
			throw {
				kind: "auth",
				code: "missing_refresh_token",
				message: "No se encontró el token de refresco de la sesión.",
			} as const
		}

		const tokens = await request<AuthTokens>({
			method: "POST",
			url: "/auth/refresh",
			data: null,
			headers: {
				Authorization: `Bearer ${refreshToken}`,
			},
			skipAuth: true,
			skipRefresh: true,
		})

		if (!sessionManager.getUser()) {
			throw {
				kind: "auth",
				code: "invalid_session",
				message: "No es posible refrescar tokens sin un usuario autenticado.",
			} as const
		}

		return tokens
	}

	public async logout(): Promise<void> {
		const accessToken = sessionManager.getAccessToken()

		if (!accessToken) return

		await request<null>({
			method: "POST",
			url: "/auth/logout",
			data: null,
			headers: {
				Authorization: `Bearer ${accessToken}`,
			},
			skipRefresh: true,
		})
	}

	public async bootstrapSession(): Promise<void> {
		if (authStore.isReady) return

		if (this.#bootstrapPromise) return this.#bootstrapPromise

		this.#bootstrapPromise = (async () => {
			authStore.isBootstrapping = true

			if (!authStore.refreshToken) {
				authStore.isReady = true
				authStore.isBootstrapping = false
				return
			}

			try {
				const tokens = await this.refresh()
				sessionManager.updateTokens(tokens)
			} catch {
				// Sesión no existe — no es un error
			}

			authStore.isReady = true
			authStore.isBootstrapping = false
		})()

		const result = await this.#bootstrapPromise
		this.#bootstrapPromise = null
		return result
	}
}

export const authService = new AuthService()
