import { PersistedState } from "runed"
import type { AuthTokens, LoginResponse, User } from "$lib/auth/types"

class AuthStore {
	#tokens = new PersistedState<AuthTokens | null>("auth-tokens", null, {
		storage: "local",
		syncTabs: false,
	})

	#user = new PersistedState<User | null>("auth-user", null, {
		storage: "local",
		syncTabs: false,
	})

	isReady = $state(false)
	isBootstrapping = $state(false)

	get tokens() {
		return this.#tokens.current
	}

	get user() {
		return this.#user.current
	}

	get accessToken() {
		return this.tokens?.accessToken ?? null
	}

	get refreshToken() {
		return this.tokens?.refreshToken ?? null
	}

	get session(): LoginResponse | null {
		if (!this.tokens || !this.user) {
			return null
		}

		return {
			accessToken: this.tokens.accessToken,
			refreshToken: this.tokens.refreshToken,
			user: this.user,
		}
	}

	isAuthenticated = () => Boolean(this.accessToken && this.refreshToken && this.user)

	setSession = (session: LoginResponse) => {
		this.#tokens.current = {
			accessToken: session.accessToken,
			refreshToken: session.refreshToken,
		}

		this.#user.current = session.user
	}

	updateTokens = (tokens: AuthTokens) => {
		if (this.user) {
			this.#tokens.current = tokens
		}

		return
	}

	clearSession = () => {
		this.#tokens.current = null
		this.#user.current = null
	}

	clearAllStores = () => {
		this.clearSession()
	}
}

export const authStore = new AuthStore()
