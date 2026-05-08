import { browser } from "$app/environment"
import { AxiosHeaders, type AxiosError } from "axios"
import { authService } from "$lib/auth/auth.service"
import { sessionManager } from "$lib/shared/auth/session.manager"
import { apiClient } from "$lib/shared/http/http"
import { refreshCoordinator } from "$lib/shared/http/refresh.coordinator"

import type { ApiRequestConfig } from "$lib/shared/http/http"

let teardownInterceptors: (() => void) | null = null

const refreshAccessToken = async (): Promise<string | null> => {
	return refreshCoordinator.run(async () => {
		try {
			const tokens = await authService.refreshOrThrow()
			sessionManager.updateTokens(tokens)
			return tokens.accessToken
		} catch {
			return null
		}
	})
}

const handleUnauthorized = async (originalConfig: ApiRequestConfig) => {
	originalConfig._retry = true

	const refreshedAccessToken = await refreshAccessToken()

	if (!refreshedAccessToken) {
		sessionManager.clearSession()
		return Promise.reject({
			kind: "auth",
			code: "session_expired",
			message: "La sesión expiró. Debes iniciar sesión nuevamente.",
			details: null,
		})
	}

	const headers = AxiosHeaders.from(originalConfig.headers as AxiosHeaders)

	headers.set("Authorization", `Bearer ${refreshedAccessToken}`)
	originalConfig.headers = headers

	return apiClient.request(originalConfig)
}

const shouldAttemptRefresh = (status: number | undefined) => {
	return status === 401 || status === 403
}

export const setupApiInterceptors = () => {
	if (teardownInterceptors) {
		return teardownInterceptors
	}

	const requestInterceptorId = apiClient.interceptors.request.use(config => {
		const requestConfig = config as ApiRequestConfig

		if (requestConfig.skipAuth) {
			return config
		}

		const accessToken = sessionManager.getAccessToken()
		if (!accessToken) return config

		const headers = AxiosHeaders.from(config.headers)
		headers.set("Authorization", `Bearer ${accessToken}`)
		config.headers = headers

		return config
	})

	const responseInterceptorId = apiClient.interceptors.response.use(
		response => response,
		async (error: AxiosError) => {
			const originalConfig = error.config as ApiRequestConfig | undefined
			const status = error.response?.status

			if (!originalConfig) return Promise.reject(error)

			if (
				browser &&
				shouldAttemptRefresh(status) &&
				window.location.pathname === "/login"
			) {
				return Promise.reject(error)
			}

			if (
				shouldAttemptRefresh(status) &&
				!originalConfig.skipRefresh &&
				!originalConfig._retry
			) {
				return handleUnauthorized(originalConfig)
			}

			return Promise.reject(error)
		}
	)

	teardownInterceptors = () => {
		apiClient.interceptors.request.eject(requestInterceptorId)
		apiClient.interceptors.response.eject(responseInterceptorId)
		teardownInterceptors = null
	}

	return teardownInterceptors
}
