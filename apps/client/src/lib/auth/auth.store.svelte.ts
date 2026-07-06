import type { User } from "$lib/auth/auth.dtos"

class AuthStore {
	user = $state<User | null>(null)
	isReady = $state(false)
	isBootstrapping = $state(false)

	isAuthenticated = () => Boolean(this.user)

	setSession = (user: User) => {
		this.user = user
	}

	clearSession = () => {
		this.user = null
	}
}

export const authStore = new AuthStore()
