import { authStore } from "$lib/auth/auth.store.svelte"
import type { AuthTokens, LoginResponse, User } from "$lib/auth/types"

class SessionManager {
	public getAccessToken(): string | null {
		return authStore.accessToken
	}

	public getRefreshToken(): string | null {
		return authStore.refreshToken
	}

	public getUser(): User | null {
		return authStore.user
	}

	public setSession(session: LoginResponse): void {
		authStore.setSession(session)
	}

	public updateTokens(tokens: AuthTokens): void {
		authStore.updateTokens(tokens)
	}

	public clearSession(): void {
		authStore.clearAllStores()
	}
}

export const sessionManager = new SessionManager()
