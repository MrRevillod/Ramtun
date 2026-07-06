import { type AxiosError } from "axios"
import { apiClient, type ApiRequestConfig } from "$lib/shared/http/http"
import { authStore } from "$lib/auth/auth.store.svelte"

export const setupApiInterceptors = () => {
	apiClient.interceptors.response.use(
		response => response,
		async (error: AxiosError) => {
			const config = error.config as ApiRequestConfig
			const status = error.response?.status

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
				await apiClient.post("/auth/refresh", null, { skipRefresh: true } as ApiRequestConfig)
				return await apiClient.request(config)
			} catch {
				authStore.clearSession()
				window.location.href = "/login"
				return Promise.reject(error)
			}
		},
	)
}
