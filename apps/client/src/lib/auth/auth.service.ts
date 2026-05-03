import { err, ok, unwrapResultOrThrow, type AppResult } from "$lib/shared/result"
import { authStore } from "$lib/auth/auth.store.svelte"
import { sessionManager } from "$lib/shared/auth/session.manager"
import { request } from "$lib/shared/http/http"
import type { AuthTokens, LoginInput, LoginResponse } from "$lib/auth/types"

class AuthService {
	#bootstrapPromise: Promise<AppResult<void>> | null = null

	public async login(input: LoginInput): Promise<AppResult<LoginResponse>> {
		return request<LoginResponse>({
			method: "POST",
			url: "/auth/login",
			data: input,
			skipAuth: true,
			skipRefresh: true,
		})
	}

	public async loginOrThrow(input: LoginInput): Promise<LoginResponse> {
		return unwrapResultOrThrow(await this.login(input))
	}

	public async refresh(): Promise<AppResult<AuthTokens>> {
		const refreshToken = sessionManager.getRefreshToken()

		if (!refreshToken) {
			return err({
				kind: "auth",
				code: "missing_refresh_token",
				message: "No se encontró el token de refresco de la sesión.",
			})
		}

		const refreshResult = await request<AuthTokens>({
			method: "POST",
			url: "/auth/refresh",
			data: null,
			headers: {
				Authorization: `Bearer ${refreshToken}`,
			},
			skipAuth: true,
			skipRefresh: true,
		})

		if (refreshResult.isErr()) return err(refreshResult.error)

		if (!sessionManager.getUser()) {
			return err({
				kind: "auth",
				code: "invalid_session",
				message: "No es posible refrescar tokens sin un usuario autenticado.",
			})
		}

		return ok(refreshResult.value)
	}

	public async refreshOrThrow(): Promise<AuthTokens> {
		return unwrapResultOrThrow(await this.refresh())
	}

	public async logout(): Promise<AppResult<void>> {
		const accessToken = sessionManager.getAccessToken()

		if (!accessToken) return ok(undefined)

		const logoutResult = await request<null>({
			method: "POST",
			url: "/auth/logout",
			data: null,
			headers: {
				Authorization: `Bearer ${accessToken}`,
			},
			skipRefresh: true,
		})

		if (logoutResult.isErr()) {
			return err(logoutResult.error)
		}

		return ok(undefined)
	}

	public async logoutOrThrow(): Promise<void> {
		return unwrapResultOrThrow(await this.logout())
	}

	public async bootstrapSession(): Promise<AppResult<void>> {
		if (authStore.isReady) {
			return ok(undefined)
		}

		if (this.#bootstrapPromise) {
			return this.#bootstrapPromise
		}

		this.#bootstrapPromise = (async () => {
			authStore.isBootstrapping = true

			if (!authStore.refreshToken) {
				authStore.isReady = true
				authStore.isBootstrapping = false
				return ok(undefined)
			}

			const refreshResult = await this.refresh()

			if (refreshResult.isOk()) {
				sessionManager.updateTokens(refreshResult.value)
			}

			authStore.isReady = true
			authStore.isBootstrapping = false

			return refreshResult.isErr() ? err(refreshResult.error) : ok(undefined)
		})()

		const result = await this.#bootstrapPromise
		this.#bootstrapPromise = null

		return result
	}
}

export const authService = new AuthService()
