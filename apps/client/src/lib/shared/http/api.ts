import axios from "axios"
import type { AxiosError, AxiosResponse } from "axios"

import { authStore } from "$lib/auth/auth.store.svelte"

export const api = axios.create({
	baseURL: "/api",
	withCredentials: true,
	headers: {
		"Content-Type": "application/json",
	},
})

api.interceptors.response.use(
	(response: AxiosResponse) => response,
	async (error: AxiosError) => {
		const config = error.config
		const status = error.response?.status

		if (!config || !status) return Promise.reject(error)
		if (status !== 401 && status !== 403) return Promise.reject(error)
		if (config.skipRefresh) return Promise.reject(error)
		if (window.location.pathname === "/login") return Promise.reject(error)

		if (config._retry) {
			authStore.clearSession()
			window.location.href = "/login"
			return Promise.reject(error)
		}

		config._retry = true

		try {
			await api.post("/auth/refresh", null, {
				skipRefresh: true,
			})

			return await api.request(config)
		} catch {
			authStore.clearSession()
			window.location.href = "/login"
			return Promise.reject(error)
		}
	}
)
