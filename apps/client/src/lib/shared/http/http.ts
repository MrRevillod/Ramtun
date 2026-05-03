import axios, { isAxiosError } from "axios"
import type { AxiosRequestConfig } from "axios"
import type { AppError } from "$lib/shared/errors"
import { ResultAsync, err, ok, type AppResult } from "$lib/shared/result"

type ApiResponse<T = unknown> = {
	code: number
	success: boolean
	message: string
	timestamp: string
	data?: T | null
	error?: unknown
}

export type ApiRequestConfig<TData = unknown> = AxiosRequestConfig<TData> & {
	skipAuth?: boolean
	skipRefresh?: boolean
	_retry?: boolean
}

export const apiClient = axios.create({
	baseURL: "/api",
	headers: {
		"Content-Type": "application/json",
	},
})

const UNKNOWN_ERROR: AppError = {
	kind: "unknown",
	message: "Ocurrio un error desconocido.",
}

const isApiResponse = (payload: unknown): payload is ApiResponse<unknown> => {
	if (!payload || typeof payload !== "object") {
		return false
	}

	const response = payload as Partial<ApiResponse<unknown>>

	return (
		typeof response.code === "number" &&
		typeof response.success === "boolean" &&
		typeof response.message === "string" &&
		typeof response.timestamp === "string"
	)
}

const mapTransportError = (error: unknown): AppError => {
	if (!isAxiosError(error)) {
		return { ...UNKNOWN_ERROR, details: error }
	}

	if (error.code === "ECONNABORTED") {
		return {
			kind: "network",
			code: "timeout",
			message: "La solicitud tardó demasiado tiempo.",
			details: error,
		}
	}

	if (!error.response) {
		return {
			kind: "network",
			code: "unknown",
			message: "No fue posible conectar con el servidor.",
			details: error,
		}
	}

	const payload = error.response.data

	if (isApiResponse(payload)) {
		return {
			kind: "http",
			status: payload.code,
			message: payload.message,
			details: payload.error ?? null,
		}
	}

	return {
		kind: "http",
		status: error.response.status,
		message: "El servidor respondió con un formato de error no esperado.",
		details: payload,
	}
}

export const request = <T>(config: ApiRequestConfig): ResultAsync<T, AppError> =>
	ResultAsync.fromPromise(
		apiClient.request<ApiResponse<T>>(config),
		mapTransportError
	).andThen((response): AppResult<T> => {
		if (!response || !isApiResponse(response.data)) {
			return err<T, AppError>({
				kind: "decode",
				code: "invalid_envelope",
				message: "Respuesta del servidor inválida.",
				details: response?.data ?? null,
			})
		}

		const payload = response.data

		if (!payload.success) {
			return err<T, AppError>({
				kind: "http",
				status: payload.code,
				message: payload.message,
				details: payload.error ?? null,
			})
		}

		return ok(payload.data as T)
	})
