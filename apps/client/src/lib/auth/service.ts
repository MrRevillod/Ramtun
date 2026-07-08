import type { UserDTO } from "$lib/users/dtos"
import type { LoginDTO } from "./dtos"

import { User } from "$lib/users/entity"
import { http } from "$lib/shared/http/client"
import { authStore } from "$lib/auth/store.svelte"
import { inlineTryAsync } from "$lib/shared/try"

class AuthService {
	private bootstrapPromise: Promise<void> | null = null

	public async login(input: LoginDTO): Promise<User> {
		const user = http.request<UserDTO>({
			method: "POST",
			url: "/auth/login",
			data: input,
			skipRefresh: true,
		})

		return user.then((user) => User.fromDTO(user))
	}

	public async logout(): Promise<void> {
		return http.request<void>({ method: "POST", url: "/auth/logout" }).catch(() => {
			authStore.clearSession()
		})
	}

	public async bootstrapSession(): Promise<void> {
		if (authStore.isReady) return
		if (this.bootstrapPromise) return this.bootstrapPromise

		this.bootstrapPromise = (async () => {
			authStore.isBootstrapping = true

			const [_, err] = await inlineTryAsync(async () => {
				const user = await http.request<UserDTO>({
					method: "GET",
					url: "/users/me",
				})
				authStore.setSession(User.fromDTO(user))
			})

			if (err) {
				authStore.clearSession()
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
